#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXAMPLE_DIR="$ROOT_DIR/example"
ADB="${ANDROID_HOME:-$HOME/Library/Android/sdk}/platform-tools/adb"
EMULATOR="${ANDROID_HOME:-$HOME/Library/Android/sdk}/emulator/emulator"
APP_ID="com.example.xue_hua_app_badge_example"
AVD_NAME="${ANDROID_AVD:-Pixel_10}"

export PATH="$(dirname "$ADB"):$(dirname "$EMULATOR"):$PATH"

log() {
  echo "[test_android_badge] $*"
}

wait_for_boot() {
  log "Waiting for device..."
  "$ADB" wait-for-device
  for _ in $(seq 1 60); do
    local boot
    boot="$("$ADB" shell getprop sys.boot_completed 2>/dev/null | tr -d '\r' || true)"
    if [[ "$boot" == "1" ]]; then
      log "Device boot completed"
      return 0
    fi
    sleep 5
  done
  log "ERROR: device did not finish booting"
  return 1
}

ensure_emulator() {
  if "$ADB" devices | awk 'NR>1 && $2=="device" {found=1} END{exit !found}'; then
    log "Android device already connected"
    return 0
  fi

  log "Starting emulator: $AVD_NAME"
  rm -f "$HOME/.android/avd/${AVD_NAME}.avd/"*.lock 2>/dev/null || true
  nohup "$EMULATOR" -avd "$AVD_NAME" -no-snapshot-load -memory 2048 -cores 2 -no-audio -no-boot-anim \
    >/tmp/xue_hua_emulator.log 2>&1 &
  wait_for_boot
}

get_device_id() {
  "$ADB" devices | awk 'NR>1 && $2=="device" {print $1; exit}'
}

install_app() {
  log "Building and installing example app"
  cd "$EXAMPLE_DIR"
  flutter pub get
  flutter build apk --debug
  "$ADB" install -r build/app/outputs/flutter-apk/app-debug.apk
}

grant_notification_permission() {
  local sdk
  sdk="$("$ADB" shell getprop ro.build.version.sdk | tr -d '\r')"
  if [[ "$sdk" -ge 33 ]]; then
    log "Granting POST_NOTIFICATIONS for API $sdk"
    "$ADB" shell appops set "$APP_ID" POST_NOTIFICATION allow || true
    "$ADB" shell pm grant "$APP_ID" android.permission.POST_NOTIFICATIONS || true
  fi
}

run_integration_test() {
  local device_id
  device_id="$(get_device_id)"
  if [[ -z "$device_id" ]]; then
    log "ERROR: no Android device found"
    return 1
  fi

  log "Running integration tests on $device_id"
  cd "$EXAMPLE_DIR"
  flutter test integration_test/badge_android_test.dart -d "$device_id"
}

verify_logcat_clean() {
  log "Checking logcat for badge errors"
  "$ADB" logcat -c
  cd "$EXAMPLE_DIR"
  flutter test integration_test/badge_android_test.dart -d "$(get_device_id)" >/dev/null

  if "$ADB" logcat -d | grep -E "Method not found|applyBadge returned false|SecurityException.*notification"; then
    log "ERROR: logcat contains badge-related errors"
    return 1
  fi

  log "No badge errors in logcat"
}

main() {
  ensure_emulator
  install_app
  grant_notification_permission
  run_integration_test
  verify_logcat_clean
  log "All Android badge checks passed"
}

main "$@"
