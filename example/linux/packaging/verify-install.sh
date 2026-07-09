#!/usr/bin/env bash
# Automated checks after building or installing the example deb.
# Full Dock badge visibility still requires manual verification on Ubuntu/GNOME.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
EXAMPLE_DIR="$(cd "${SCRIPT_DIR}/../.." && pwd)"
DEB="${EXAMPLE_DIR}/dist/xue-hua-app-badge-example_1.0.0_amd64.deb"
TEST_ROOT="${TEST_ROOT:-/tmp/xue_hua_app_badge_deb_test}"

if [[ ! -f "${DEB}" ]]; then
  echo "error: deb not found at ${DEB}; run ./build-deb.sh first" >&2
  exit 1
fi

echo "==> Extracting deb to ${TEST_ROOT}"
rm -rf "${TEST_ROOT}"
mkdir -p "${TEST_ROOT}"
dpkg-deb -x "${DEB}" "${TEST_ROOT}"

DESKTOP="${TEST_ROOT}/usr/share/applications/com.example.xue_hua_app_badge.desktop"
BINARY="${TEST_ROOT}/usr/lib/xue_hua_app_badge_example/xue_hua_app_badge_example"
ICON="${TEST_ROOT}/usr/share/icons/hicolor/256x256/apps/com.example.xue_hua_app_badge.png"

for f in "${DESKTOP}" "${BINARY}" "${ICON}"; do
  [[ -e "$f" ]] || { echo "error: missing packaged file: $f" >&2; exit 1; }
done

grep -q 'Exec=/usr/lib/xue_hua_app_badge_example/xue_hua_app_badge_example' "${DESKTOP}"
grep -q 'StartupWMClass=xue_hua_app_badge_example' "${DESKTOP}"
grep -q 'Icon=com.example.xue_hua_app_badge' "${DESKTOP}"
echo "==> Desktop entry fields OK"

echo "==> Launching app with simulated menu environment"
export GIO_LAUNCHED_DESKTOP_FILE="${DESKTOP}"
export DISPLAY="${DISPLAY:-:0}"
cd "${TEST_ROOT}/usr/lib/xue_hua_app_badge_example"
./xue_hua_app_badge_example >/tmp/xue_hua_app_badge_verify.log 2>&1 &
APP_PID=$!
cleanup() { kill "${APP_PID}" 2>/dev/null || true; }
trap cleanup EXIT
sleep 4

if ! ps -p "${APP_PID}" >/dev/null; then
  echo "error: app exited early" >&2
  cat /tmp/xue_hua_app_badge_verify.log >&2
  exit 1
fi

tr '\0' '\n' < "/proc/${APP_PID}/environ" | grep -q 'GIO_LAUNCHED_DESKTOP_FILE=.*com.example.xue_hua_app_badge.desktop'
echo "==> GIO_LAUNCHED_DESKTOP_FILE present in process environment"

echo "==> Emitting Unity LauncherEntry Update via session D-Bus"
gdbus emit --session \
  --object-path /com/canonical/unity/launcherentry/1 \
  --signal com.canonical.Unity.LauncherEntry.Update \
  string:"application://com.example.xue_hua_app_badge.desktop" \
  dict:string:int64:3,string:boolean:true
echo "D-Bus Update signal emitted for application://com.example.xue_hua_app_badge.desktop"

echo
echo "Automated checks passed."
echo "Manual step (Ubuntu/GNOME): install deb, launch from app menu, pin to dock, click +1/-1/Clear and confirm dock badge changes."
