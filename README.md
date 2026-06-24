# xue_hua_app_badge

跨平台 Flutter 应用角标（Badge）插件。核心逻辑在 **Rust** 中实现，通过 [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) v2 与 Dart 通信；Android 侧仅保留极薄的 Kotlin 引导层（Context 初始化 + ShortcutBadger）。

## 特性

- 统一 API：`XueHuaAppBadge.initialize()` / `XueHuaAppBadge.set(count)` / `XueHuaAppBadge.remove()` / `XueHuaAppBadge.requestPermission()` / `XueHuaAppBadge.isPermissionGranted()`
- 底层逻辑在 Rust 中按平台条件编译（`#[cfg(target_os = "...")]`）
- 支持 Android、iOS、macOS、Windows、Linux 五端（Cargokit 自动编译 Rust）
- 数字超过 99 时：macOS 显示 `99+` 文本；其他平台显示 `99`

## 平台支持

| 平台 | 机制 | 状态 |
|------|------|------|
| **macOS** | `NSApplication.dockTile.setBadgeLabel` | ✅ 已实现 |
| **Windows** | `ITaskbarList3::SetOverlayIcon`（COM + GDI） | ✅ 已实现 |
| **Linux** | D-Bus `com.canonical.Unity.LauncherEntry`（Ubuntu / GNOME / KDE Plasma） | ✅ 已实现 |
| **iOS** | iOS 16+ `UNUserNotificationCenter.setBadgeCount`；低版本 `UIApplication.applicationIconBadgeNumber` | ✅ 已实现 |
| **Android** | JNI + ShortcutBadger + NotificationChannel 静默回退（API 26+） | ✅ 已实现 |

## 安装

```yaml
dependencies:
  xue_hua_app_badge: ^1.0.1
```

### 环境要求

- Flutter >= 3.3.0，Dart SDK ^3.12.2
- **Rust** 工具链（[rustup](https://rustup.rs/)）
- 各平台常规 Flutter 开发环境
- 修改 Rust API 后：`cargo install flutter_rust_bridge_codegen`

## 快速开始

```dart
import 'package:flutter/material.dart';
import 'package:xue_hua_app_badge/xue_hua_app_badge.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await XueHuaAppBadge.initialize();
  runApp(const MyApp());
}

// iOS 16+ / Android 13+ 建议先请求权限
if (!XueHuaAppBadge.isPermissionGranted()) {
  XueHuaAppBadge.requestPermission();
}

XueHuaAppBadge.set(5);
XueHuaAppBadge.remove();
```

### iOS / Android：Badge 权限

插件提供显式权限 API（**不会在 `set()` 时自动弹窗**）：

| 平台 | 权限 | 说明 |
|------|------|------|
| **iOS 16+** | 通知 Badge 授权 | `requestPermission()` 弹出系统对话框 |
| **Android 13+** | `POST_NOTIFICATIONS` | 通知回退路径需要；API 32 及以下直接返回 `true` |
| **macOS / Windows / Linux** | 无 | 恒返回 `true` |

```dart
if (!XueHuaAppBadge.isPermissionGranted()) {
  final granted = XueHuaAppBadge.requestPermission();
  if (!granted) {
    // 用户拒绝，可提示前往系统设置
  }
}
XueHuaAppBadge.set(count);
```

### Linux：Desktop 文件 ID

Linux 通过 Unity LauncherEntry 协议更新任务栏角标，需要能解析 `.desktop` 文件 ID：

1. 从桌面快捷方式启动时，系统自动设置 `GIO_LAUNCHED_DESKTOP_FILE`
2. 否则在 Linux runner 中设置 `GAPPLICATION_ID`（example 已配置）：

```cpp
g_setenv("GAPPLICATION_ID", APPLICATION_ID, TRUE);
```

### Windows：窗口句柄（可选）

```dart
XueHuaAppBadge.set(3, windowHandle: hwnd);
```

未传入时 Rust 回退到 `GetActiveWindow()`。

### 错误处理

```dart
try {
  XueHuaAppBadge.set(count);
} catch (e) {
  debugPrint('Badge error: $e');
}
```

## 架构

```
Dart (XueHuaAppBadge.initialize → RustLib.init)
    ↓ flutter_rust_bridge
rust/src/api/badge.rs
    ↓ #[cfg]
platform/win_impl.rs      ← Windows ITaskbarList3
platform/macos_impl.rs    ← macOS NSDockTile
platform/ios_impl.rs      ← iOS UNUserNotificationCenter / UIApplication
platform/android_impl.rs  ← JNI → BadgeHelper.kt
platform/linux_impl.rs    ← zbus Unity LauncherEntry
```

AppKit / UIKit 调用通过 `dispatch2` 派发到主线程。Android 通过 `XueHuaAppBadgePlugin` 在启动时注入 `Context`。

## 本地开发

```bash
cd example && flutter pub get
flutter run -d macos
flutter run -d windows
flutter run -d linux
flutter run -d ios
flutter run -d android
```

修改 Rust API 后：

```bash
flutter_rust_bridge_codegen generate
cd rust && cargo check
```

## 已知限制

- **Windows**：小任务栏模式下 `SetOverlayIcon` 无效
- **Windows**：`GetActiveWindow()` 对多窗口不可靠
- **macOS**：仅 Dock 角标
- **iOS**：iOS 16+ 需先调用 `requestPermission()`；未授权时 `set()` 可能失败
- **Android**：Android 13+ 建议先调用 `requestPermission()`；部分 Launcher 不支持 ShortcutBadger
- **Linux**：仅 Unity LauncherEntry 协议；i3/sway 等极简 WM 不支持

## 依赖概览

| 组件 | 用途 |
|------|------|
| `flutter_rust_bridge` 2.12.0 | Dart ↔ Rust |
| `windows` 0.62 | Windows COM / GDI |
| `objc2` + AppKit / UIKit / UserNotifications | macOS / iOS |
| `zbus` 5.x | Linux D-Bus |
| `jni` 0.22 | Android JNI |
| ShortcutBadger 1.1.22 | Android 厂商 Launcher |
| Cargokit | 跨平台 Rust 构建 |

## License

见仓库根目录 LICENSE 文件（如有）。
