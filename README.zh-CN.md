[English](README.md) | **简体中文**

# xue_hua_app_badge

跨平台 Flutter 应用角标（Badge）插件。核心逻辑在 **Rust** 中实现，通过 [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) v2 与 Dart 通信。Android 侧保留薄 Kotlin 层：[`ndk-context`](https://docs.rs/ndk-context) 初始化 Context/JavaVM、`BadgeHelper`（ShortcutBadger + Notification 回退）、`PermissionHelper`（Android 13+ 权限）。

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
| **Android** | ndk-context + JNI + ShortcutBadger + NotificationChannel 静默回退（API 26+） | ✅ 已实现 |

## 安装

```yaml
dependencies:
  xue_hua_app_badge: ^1.0.5
```

### 环境要求

- Flutter >= 3.3.0，Dart SDK ^3.12.2
- 各平台常规 Flutter 开发环境
- 当当前 crate hash 对应的预编译产物已经发布时，大多数插件使用者**不需要**安装 Rust
- 维护者和贡献者仍应安装 [rustup](https://rustup.rs/) 及 `stable` 工具链（见下方 [Rust 编译要求](#rust-编译要求)）
- 如果你在本地开发尚未发布的 Rust 改动，新的 crate hash 在预编译产物发布前仍会走源码编译
- 修改 Rust 公开 API 后：`cargo install flutter_rust_bridge_codegen`

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
platform/android_impl.rs  ← ndk-context → JNI → BadgeHelper.kt
platform/linux_impl.rs    ← zbus Unity LauncherEntry
```

AppKit / UIKit 调用通过 `dispatch2` 派发到主线程。Android 上下文由 [`ndk-context`](https://docs.rs/ndk-context) 提供；`XueHuaAppBadgePlugin` 在启动时调用 `initialize_android_context`，并管理 Activity 权限生命周期。

## Rust 编译要求

本插件通过 [Cargokit](https://github.com/irondash/cargokit) 在 `flutter run` / `flutter build` 时优先使用已发布的签名预编译产物，并在需要时回退到本地 Rust 编译，**无需手动运行 `cargo ndk`**。

### 插件使用者

- Flutter >= 3.3.0，Dart ^3.12.2
- 目标平台对应的常规 Flutter 开发环境
- 如果当前机器**没有**安装 `rustup`，Cargokit 会尝试下载当前 crate hash 对应的签名预编译产物
- 如果当前机器**已经**安装了 `rustup`，Cargokit 按默认行为仍优先本地编译，除非你显式开启预编译下载

### 已安装 Rust 时强制使用预编译产物

在 Flutter 应用根目录创建 `cargokit_options.yaml`：

```yaml
use_precompiled_binaries: true
```

适用场景：
- 机器上已经安装了 Rust，但你不想为这个插件重新编译
- 你想验证仓库中已经发布的预编译产物是否可用

### 维护者 / 贡献者

- 安装 [rustup](https://rustup.rs/)，确保 `stable` 工具链在 PATH 中
- 首次源码构建时，Cargokit 会通过 `rustup target add` 自动安装所需交叉编译 target
- 当你修改 Rust 源码、`Cargo.toml`、`Cargo.lock`、`build.rs` 或 `rust/cargokit.yaml` 时，crate hash 会变化；在新的预编译产物发布前，仍需要本地源码编译
- 修改 Rust 公开 API 后需运行：

```bash
flutter_rust_bridge_codegen generate
cd rust && cargo check   # 仅验证当前 host 平台
```

### 按平台

| 平台 | 额外要求 |
|------|----------|
| **Android** | Android SDK + **NDK**；宿主 App 的 `build.gradle` 必须设置 `android.ndkVersion`（否则 Cargokit 报错）；构建时编译 `armv7-linux-androideabi`、`aarch64-linux-android`、`i686-linux-android`、`x86_64-linux-android` |
| **iOS / macOS** | Xcode + Apple 工具链 |
| **Windows** | Visual Studio Build Tools（MSVC） |
| **Linux** | `gcc`/`clang`；运行时依赖系统 D-Bus（zbus） |

### 常见构建失败

| 错误信息 | 处理方式 |
|----------|----------|
| `rustup not found` | 插件使用者通常可以直接依赖已发布的预编译产物；插件维护者则应安装 rustup 并重启终端 |
| `Please set 'android.ndkVersion'` | 在 App 级 `android/app/build.gradle` 配置 NDK 版本 |
| `android context was not initialized` | 确保通过 `pubspec.yaml` 正常依赖插件（无需手动 MethodChannel） |
| 修改 Rust API 后 Dart 侧报错 | 重新运行 `flutter_rust_bridge_codegen generate` |

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
| `ndk-context` 0.1.1 | Android Context / JavaVM |
| ShortcutBadger 1.1.22 | Android 厂商 Launcher |
| Cargokit | 跨平台 Rust 构建 |

## License

见仓库根目录 LICENSE 文件。
