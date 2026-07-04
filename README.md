**English** | [简体中文](README.zh-CN.md)

# xue_hua_app_badge

Cross-platform Flutter app badge plugin. Core logic is implemented in **Rust** and exposed to Dart via [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) v2. On Android, a thin Kotlin layer handles [`ndk-context`](https://docs.rs/ndk-context) initialization, `BadgeHelper` (ShortcutBadger + notification fallback), and `PermissionHelper` (Android 13+ permissions).

## Features

- Unified API: `XueHuaAppBadge.initialize()` / `XueHuaAppBadge.set(count)` / `XueHuaAppBadge.remove()` / `XueHuaAppBadge.requestPermission()` / `XueHuaAppBadge.isPermissionGranted()`
- Platform-specific logic compiled in Rust via `#[cfg(target_os = "...")]`
- Android, iOS, macOS, Windows, and Linux (Rust built automatically by Cargokit)
- Counts above 99: macOS shows `99+` text; other platforms cap at `99`

## Platform Support

| Platform | Mechanism | Status |
|----------|-----------|--------|
| **macOS** | `NSApplication.dockTile.setBadgeLabel` | Implemented |
| **Windows** | `ITaskbarList3::SetOverlayIcon` (COM + GDI) | Implemented |
| **Linux** | D-Bus `com.canonical.Unity.LauncherEntry` (Ubuntu / GNOME / KDE Plasma) | Implemented |
| **iOS** | iOS 16+ `UNUserNotificationCenter.setBadgeCount`; older `UIApplication.applicationIconBadgeNumber` | Implemented |
| **Android** | ndk-context + JNI + ShortcutBadger + silent NotificationChannel fallback (API 26+) | Implemented |

## Installation

```yaml
dependencies:
  xue_hua_app_badge: ^1.0.2
```

### Requirements

- Flutter >= 3.3.0, Dart SDK ^3.12.2
- [rustup](https://rustup.rs/) with the `stable` toolchain (see [Rust Build Requirements](#rust-build-requirements))
- Standard Flutter tooling for each target platform
- After changing the Rust public API: `cargo install flutter_rust_bridge_codegen`

## Quick Start

```dart
import 'package:flutter/material.dart';
import 'package:xue_hua_app_badge/xue_hua_app_badge.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await XueHuaAppBadge.initialize();
  runApp(const MyApp());
}

// Recommended on iOS 16+ / Android 13+
if (!XueHuaAppBadge.isPermissionGranted()) {
  XueHuaAppBadge.requestPermission();
}

XueHuaAppBadge.set(5);
XueHuaAppBadge.remove();
```

### iOS / Android: Badge Permissions

The plugin exposes explicit permission APIs (**no automatic prompt on `set()`**):

| Platform | Permission | Notes |
|----------|------------|-------|
| **iOS 16+** | Notification badge authorization | `requestPermission()` shows the system dialog |
| **Android 13+** | `POST_NOTIFICATIONS` | Required for the notification fallback path; API 32 and below always return `true` |
| **macOS / Windows / Linux** | None | Always return `true` |

```dart
if (!XueHuaAppBadge.isPermissionGranted()) {
  final granted = XueHuaAppBadge.requestPermission();
  if (!granted) {
    // User denied — guide them to system settings
  }
}
XueHuaAppBadge.set(count);
```

### Linux: Desktop File ID

Linux uses the Unity LauncherEntry protocol and needs a resolvable `.desktop` file ID:

1. When launched from a desktop shortcut, the system sets `GIO_LAUNCHED_DESKTOP_FILE`
2. Otherwise set `GAPPLICATION_ID` in the Linux runner (configured in the example app):

```cpp
g_setenv("GAPPLICATION_ID", APPLICATION_ID, TRUE);
```

### Windows: Window Handle (Optional)

```dart
XueHuaAppBadge.set(3, windowHandle: hwnd);
```

When omitted, Rust falls back to `GetActiveWindow()`.

### Error Handling

```dart
try {
  XueHuaAppBadge.set(count);
} catch (e) {
  debugPrint('Badge error: $e');
}
```

## Architecture

```
Dart (XueHuaAppBadge.initialize → RustLib.init)
    ↓ flutter_rust_bridge
rust/src/api/badge.rs
    ↓ #[cfg]
platform/win_impl.rs      ← Windows ITaskbarList3
platform/macos_impl.rs    ← macOS NSDockTile
platform/ios_impl.rs      ← iOS UNUserNotificationCenter / UIApplication
platform/android_impl.rs  ← ndk-context → JNI → BadgeHelper.kt
platform/linux_impl.rs    ← zbus Unity LauncherEntry
```

AppKit / UIKit calls are dispatched to the main thread via `dispatch2`. On Android, context is provided by [`ndk-context`](https://docs.rs/ndk-context); `XueHuaAppBadgePlugin` calls `initialize_android_context` at startup and manages the Activity lifecycle for permissions.

## Rust Build Requirements

This plugin uses [Cargokit](https://github.com/irondash/cargokit) to compile Rust automatically during `flutter run` / `flutter build`. **You do not need to run `cargo ndk` manually.**

### All Platforms

- Install [rustup](https://rustup.rs/) and ensure the `stable` toolchain is on your PATH
- Flutter >= 3.3.0, Dart ^3.12.2
- On first build, Cargokit installs required cross-compilation targets via `rustup target add`
- After changing the Rust public API:

```bash
flutter_rust_bridge_codegen generate
cd rust && cargo check   # validates the host platform only
```

### Per Platform

| Platform | Additional Requirements |
|----------|-------------------------|
| **Android** | Android SDK + **NDK**; host app `build.gradle` must set `android.ndkVersion` (Cargokit fails otherwise); builds `armv7-linux-androideabi`, `aarch64-linux-android`, `i686-linux-android`, `x86_64-linux-android` |
| **iOS / macOS** | Xcode + Apple toolchain |
| **Windows** | Visual Studio Build Tools (MSVC) |
| **Linux** | `gcc`/`clang`; system D-Bus required at runtime (zbus) |

### Common Build Failures

| Error | Fix |
|-------|-----|
| `rustup not found` | Install rustup and restart your terminal |
| `Please set 'android.ndkVersion'` | Set NDK version in the app-level `android/app/build.gradle` |
| `android context was not initialized` | Add the plugin via `pubspec.yaml` (no manual MethodChannel needed) |
| Dart errors after Rust API changes | Re-run `flutter_rust_bridge_codegen generate` |

## Known Limitations

- **Windows**: `SetOverlayIcon` does not work in small taskbar mode
- **Windows**: `GetActiveWindow()` is unreliable with multiple windows
- **macOS**: Dock badge only
- **iOS**: iOS 16+ requires `requestPermission()` first; `set()` may fail without authorization
- **Android**: Android 13+ should call `requestPermission()` first; some launchers do not support ShortcutBadger
- **Linux**: Unity LauncherEntry protocol only; minimal WMs like i3/sway are not supported

## Dependencies

| Component | Purpose |
|-----------|---------|
| `flutter_rust_bridge` 2.12.0 | Dart ↔ Rust |
| `windows` 0.62 | Windows COM / GDI |
| `objc2` + AppKit / UIKit / UserNotifications | macOS / iOS |
| `zbus` 5.x | Linux D-Bus |
| `jni` 0.22 | Android JNI |
| `ndk-context` 0.1.1 | Android Context / JavaVM |
| ShortcutBadger 1.1.22 | Android OEM launchers |
| Cargokit | Cross-platform Rust builds |

## License

See the LICENSE file in the repository root.
