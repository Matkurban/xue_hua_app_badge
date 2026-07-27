[English](README.md) | **简体中文**

# xue_hua_app_badge

跨平台 Flutter 应用角标（Badge）插件。采用原生 **MethodChannel** 架构在 Android、iOS、macOS、Windows 和 Linux 平台完整实现。

## 特性

- 统一单例类 API：`XueHuaAppBadge.instance` (`set`, `remove`, `requestPermission`, `isPermissionGranted`, `isSupported`)
- 全平台原生实现（Android, iOS, macOS, Windows, Linux）。
- iOS 与 macOS 支持 **Swift Package Manager (SPM)** 及 CocoaPods。
- Android 端采用包名 `com.kurban.xue_hua_app_badge` 及 **Kotlin DSL (`.kts`)** 构建脚本。
- 角标数值超过 99 时：macOS 显示 `99+`，其他平台限制在 `99`。

## 平台支持情况

| 平台 | 实现机制 | 状态 |
|------|----------|------|
| **macOS** | `NSApplication.dockTile.badgeLabel` | 原生 Swift 实现 (SPM/CocoaPods) |
| **Windows** | `ITaskbarList3::SetOverlayIcon` (C++ Win32 COM + GDI) | 原生 C++ 实现 |
| **Linux** | D-Bus `com.canonical.Unity.LauncherEntry` (C++ GTK + GDBus) | 原生 C++ 实现 |
| **iOS** | iOS 16+ `UNUserNotificationCenter.setBadgeCount`；旧版本 `UIApplication.applicationIconBadgeNumber` | 原生 Swift 实现 (SPM/CocoaPods) |
| **Android** | `ShortcutBadger` + 静音 `NotificationChannel` 保底 (API 26+) | 原生 Kotlin 实现 (.kts) |

## 安装

```yaml
dependencies:
  xue_hua_app_badge: ^1.0.9
```

## 快速开始

```dart
import 'package:flutter/material.dart';
import 'package:xue_hua_app_badge/xue_hua_app_badge.dart';

void main() {
  runApp(const MyApp());
}

// 建议在 iOS 16+ / Android 13+ 先进行权限检查
if (!await XueHuaAppBadge.instance.isPermissionGranted()) {
  await XueHuaAppBadge.instance.requestPermission();
}

await XueHuaAppBadge.instance.set(5);
await XueHuaAppBadge.instance.remove();
```

### iOS / Android 权限管理

插件提供明确的权限申请接口（**不会在 `set()` 时自动弹窗**）：

| 平台 | 所需权限 | 说明 |
|------|----------|------|
| **iOS 16+** | 通知角标授权 | `requestPermission()` 会展示系统权限弹窗 |
| **Android 13+** | `POST_NOTIFICATIONS` | 通知保底路径所需权限 |
| **macOS / Windows / Linux** | 无需权限 | 始终返回 `true` |

```dart
if (!await XueHuaAppBadge.instance.isPermissionGranted()) {
  final granted = await XueHuaAppBadge.instance.requestPermission();
  if (!granted) {
    // 用户拒绝权限 — 引导前往系统设置开启
  }
}
await XueHuaAppBadge.instance.set(count);
```

## 架构

```
Dart (XueHuaAppBadge.instance.set / remove / requestPermission)
    ↓ MethodChannel ('xue_hua_app_badge')
--------------------------------------------------
Android (Kotlin)  ← ShortcutBadger + NotificationChannel
iOS (Swift)       ← UNUserNotificationCenter / UIApplication
macOS (Swift)     ← NSApp.dockTile
Windows (C++)     ← ITaskbarList3::SetOverlayIcon (GDI)
Linux (C++)       ← GDBus Unity LauncherEntry Update
```

## 许可证

详见项目根目录 LICENSE 文件。
