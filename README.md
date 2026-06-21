# xue_hua_app_badge

跨平台 Flutter 应用角标（Badge）插件，采用**混合架构**：

- **Android / iOS / macOS**：原生 MethodChannel，直接调用各平台 API
- **Windows / Linux**：Rust 核心 + [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) v2

## 特性

- 统一 API：`XueHuaAppBadge.set(count)` / `XueHuaAppBadge.remove()` / `XueHuaAppBadge.requestPermission()` / `XueHuaAppBadge.isPermissionGranted()`
- 五端支持，各平台使用最适合的实现方式
- 数字超过 99 时：macOS 显示 `99+`；其他平台显示 `99`

## 平台支持

| 平台 | 机制 | 实现方式 |
|------|------|----------|
| **Android** | ShortcutBadger + NotificationChannel 回退 | Kotlin MethodChannel |
| **iOS** | UNUserNotificationCenter / UIApplication | Swift MethodChannel |
| **macOS** | NSApplication.dockTile.badgeLabel | Swift MethodChannel |
| **Windows** | ITaskbarList3::SetOverlayIcon | Rust (FRB) |
| **Linux** | Unity LauncherEntry D-Bus | Rust (FRB) |

## 安装

```yaml
dependencies:
  xue_hua_app_badge:
    path: ../xue_hua_app_badge
```

### 环境要求

- Flutter >= 3.3.0，Dart SDK ^3.12.2
- **Rust** 工具链（仅 Windows / Linux 桌面端需要）
- 各平台常规 Flutter 开发环境

## 快速开始

```dart
import 'package:flutter/material.dart';
import 'package:xue_hua_app_badge/xue_hua_app_badge.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await initBadgePlugin(); // Windows/Linux 会初始化 RustLib
  runApp(const MyApp());
}

await XueHuaAppBadge.set(5);
await XueHuaAppBadge.remove();

// iOS 16+ / Android 13+ 建议先请求权限
if (!await XueHuaAppBadge.isPermissionGranted()) {
  final granted = await XueHuaAppBadge.requestPermission();
}
await XueHuaAppBadge.set(count);
```

## 架构

```
Dart (XueHuaAppBadge)
    ├── Platform.isAndroid / iOS / macOS
    │       └── MethodChannel → 原生 Kotlin / Swift
    └── Platform.isWindows / Linux
            └── flutter_rust_bridge → Rust
                    ├── win_impl.rs   (ITaskbarList3)
                    └── linux_impl.rs (Unity LauncherEntry)
```

## 本地开发

```bash
cd example && flutter pub get
flutter run -d android
flutter run -d ios
flutter run -d macos
flutter run -d windows
flutter run -d linux
```

Android 自动化测试：

```bash
./tool/test_android_badge.sh
```

## 已知限制

- **Windows**：小任务栏模式下 `SetOverlayIcon` 无效
- **iOS**：iOS 16+ 需先调用 `requestPermission()`
- **Android**：Android 13+ 需先调用 `requestPermission()`；部分 Launcher 不支持 ShortcutBadger
- **Linux**：仅 Unity LauncherEntry 协议

## License

见仓库根目录 LICENSE 文件。
