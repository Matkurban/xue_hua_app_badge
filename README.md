**English** | [简体中文](README.zh-CN.md)

# xue_hua_app_badge

Cross-platform Flutter app badge plugin. Implemented using standard platform **MethodChannel** architecture natively across Android, iOS, macOS, Windows, and Linux.

## Features

- Unified Singleton API: `XueHuaAppBadge.instance` (`set`, `remove`, `requestPermission`, `isPermissionGranted`, `isSupported`)
- Native implementations for Android, iOS, macOS, Windows, and Linux.
- Supports **Swift Package Manager (SPM)** & CocoaPods on iOS / macOS.
- Android uses **Kotlin DSL (`.kts`)** build configuration under package `com.kurban.xue_hua_app_badge`.
- Counts above 99: macOS shows `99+` text; other platforms cap at `99`.

## Platform Support

| Platform    | Mechanism                                                                                                                                                      | Status                              |
| ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------- |
| **macOS**   | `NSApplication.dockTile.badgeLabel`                                                                                                                            | Implemented (Swift + SPM/CocoaPods) |
| **Windows** | `ITaskbarList3::SetOverlayIcon` (C++ Win32 COM + GDI)                                                                                                          | Implemented                         |
| **Linux**   | D-Bus `com.canonical.Unity.LauncherEntry` (C++ GTK + GDBus)                                                                                                    | Implemented                         |
| **iOS**     | iOS 16+ `UNUserNotificationCenter.setBadgeCount`; older `UIApplication.applicationIconBadgeNumber`                                                             | Implemented (Swift + SPM/CocoaPods) |
| **Android** | Per-OEM official APIs (Honor / Huawei / Xiaomi / OPPO / vivo / Meizu) + `ShortcutBadger` for other launchers + silent `NotificationChannel` fallback (API 26+) | Implemented (Kotlin DSL)            |

## Installation

```yaml
dependencies:
  xue_hua_app_badge: ^2.1.0
```

## Quick Start

```dart
import 'package:flutter/material.dart';
import 'package:xue_hua_app_badge/xue_hua_app_badge.dart';

void main() {
  runApp(const MyApp());
}

// Recommended on iOS 16+ / Android 13+
if (!await XueHuaAppBadge.instance.isPermissionGranted()) {
  await XueHuaAppBadge.instance.requestPermission();
}

await XueHuaAppBadge.instance.set(5);
await XueHuaAppBadge.instance.remove();
```

### iOS / Android: Badge Permissions

The plugin exposes explicit permission APIs (**no automatic prompt on `set()`**):

| Platform                    | Permission                       | Notes                                                                                                                                                  |
| --------------------------- | -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **iOS 16+**                 | Notification badge authorization | `requestPermission()` shows the system dialog                                                                                                          |
| **Android 13+**             | `POST_NOTIFICATIONS`             | Required for the notification fallback path. OEM launcher permissions are merged from the plugin manifest; the host app does not need to declare them. |
| **macOS / Windows / Linux** | None                             | Always return `true`                                                                                                                                   |

```dart
if (!await XueHuaAppBadge.instance.isPermissionGranted()) {
  final granted = await XueHuaAppBadge.instance.requestPermission();
  if (!granted) {
    // User denied — guide them to system settings
  }
}
await XueHuaAppBadge.instance.set(count);
```

### Android OEM notes

- **Honor / Huawei:** numeric badges use the launcher ContentProvider. Opening the app or dismissing notifications does **not** clear them; call `remove()` or `set(0)`.
- **Xiaomi / Redmi / POCO:** the badge is tied to notifications (progress and ongoing notifications are ignored).
- **vivo / iQOO:** the user must enable the per-app “desktop badge” switch in system settings.
- **OPPO / OnePlus / realme:** numeric badges are whitelist-gated; other apps typically only get a notification dot.
- **Meizu:** third-party apps often only get a notification dot; Flyme 10+ may accept a numeric provider call.

## Architecture

```
Dart (XueHuaAppBadge.instance.set / remove / requestPermission)
    ↓ MethodChannel ('xue_hua_app_badge')
--------------------------------------------------
Android (Kotlin)  ← Honor/Huawei/Xiaomi/OPPO/vivo/Meizu official APIs + ShortcutBadger + NotificationChannel
iOS (Swift)       ← UNUserNotificationCenter / UIApplication
macOS (Swift)     ← NSApp.dockTile
Windows (C++)     ← ITaskbarList3::SetOverlayIcon (GDI)
Linux (C++)       ← GDBus Unity LauncherEntry Update
```

## License

See the LICENSE file in the repository root.
