# xue_hua_app_badge

跨平台 Flutter 应用角标（Badge）插件。核心逻辑在 **Rust** 中实现，通过 [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) v2 与 Dart 通信；Flutter 侧仅作为薄封装，不依赖各平台原生 Method Channel（Swift/Kotlin）。

## 特性

- 统一 API：`XueHuaAppBadge.set(count)` / `XueHuaAppBadge.remove()`
- 底层逻辑全部在 Rust 中，按平台条件编译（`#[cfg(target_os = "...")]`）
- 支持 Android、iOS、macOS、Windows、Linux 五端构建（Cargokit 自动编译 Rust）
- 桌面端已可用：macOS Dock 角标、Windows 任务栏叠加图标

## 平台支持

| 平台 | 机制 | 状态 |
|------|------|------|
| **macOS** | `NSApplication.dockTile.setBadgeLabel` | ✅ 已实现 |
| **Windows** | `ITaskbarList3::SetOverlayIcon`（COM + GDI 绘制数字图标） | ✅ 已实现 |
| **Linux** | D-Bus `Unity.LauncherEntry`（zbus） | 🚧 待实现 |
| **iOS** | `UIApplication.applicationIconBadgeNumber` | 🚧 待实现 |
| **Android** | JNI + Launcher Broadcast / ShortcutBadger | 🚧 待实现 |

## 安装

在 `pubspec.yaml` 中添加依赖：

```yaml
dependencies:
  xue_hua_app_badge:
    path: ../xue_hua_app_badge   # 或发布后使用版本号
```

### 环境要求

- Flutter >= 3.3.0，Dart SDK ^3.12.2
- **Rust** 工具链（[rustup](https://rustup.rs/)）
- 各平台常规 Flutter 开发环境（Xcode、Android NDK、Visual Studio 等）
- 修改 Rust API 后需安装 codegen：

```bash
cargo install flutter_rust_bridge_codegen
```

## 快速开始

```dart
import 'package:flutter/material.dart';
import 'package:xue_hua_app_badge/xue_hua_app_badge.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  runApp(const MyApp());
}

// 设置角标
XueHuaAppBadge.set(5);

// 清除角标
XueHuaAppBadge.remove();

// 递增（超过 99 显示 "99+"）
XueHuaAppBadge.set(100);
```

### Windows：窗口句柄（可选）

Windows 需要 HWND 才能将叠加图标绑定到任务栏按钮。未传入时，Rust 侧会回退到 `GetActiveWindow()`；多窗口应用建议显式传入：

```dart
XueHuaAppBadge.set(3, windowHandle: hwnd);
```

`windowHandle` 为原生 `HWND` 的整数值（`int`）。可通过 Win32 API、`win32` 包等方式获取 Flutter 窗口句柄。

### 错误处理

Rust API 返回 `Result<(), String>`。FRB 同步调用在失败时会抛出异常，请用 `try/catch` 包裹：

```dart
try {
  XueHuaAppBadge.set(count);
} catch (e) {
  debugPrint('Badge error: $e');
}
```

未实现的平台会返回类似 `Linux badge support is not implemented yet` 的错误。

## 架构

```
Dart (XueHuaAppBadge)
    ↓ flutter_rust_bridge
rust/src/api/badge.rs          ← 统一入口 set_badge / remove_badge
    ↓ #[cfg] 分发
platform/win_impl.rs           ← Windows ITaskbarList3
platform/macos_impl.rs         ← macOS NSDockTile
platform/{linux,ios,android}_impl.rs  ← 占位，Phase 2
```

macOS 的 AppKit API 必须在主线程调用；插件内部通过 `dispatch2` 自动派发到主队列。

## 项目结构

```
xue_hua_app_badge/
├── lib/
│   ├── xue_hua_app_badge.dart      # 公开 Dart API
│   └── src/rust/                   # FRB 生成的绑定（勿手改）
├── rust/
│   ├── Cargo.toml
│   └── src/
│       ├── api/badge.rs            # FRB 暴露的 Rust API
│       └── platform/               # 各平台实现
├── cargokit/                       # 跨平台 Rust 构建工具
├── android/ ios/ macos/ linux/ windows/
├── example/                        # 示例应用
└── flutter_rust_bridge.yaml        # FRB codegen 配置
```

## 本地开发

### 运行示例

```bash
cd example
flutter pub get
flutter run -d macos    # macOS Dock 角标
flutter run -d windows  # Windows 任务栏叠加图标
```

### 修改 Rust API 后重新生成绑定

```bash
flutter_rust_bridge_codegen generate
```

配置见 `flutter_rust_bridge.yaml`：

- Rust 输入：`crate::api`（`rust/src/api/` 目录）
- Dart 输出：`lib/src/rust/`

### 仅检查 Rust 编译

```bash
cd rust && cargo check
```

## 已知限制

- **Windows 小任务栏**：用户开启「使用小任务栏按钮」时，`SetOverlayIcon` 会被系统忽略（[Microsoft 文档](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setoverlayicon)）。
- **Windows HWND**：`GetActiveWindow()` 回退方案对多窗口场景不可靠。
- **macOS**：仅支持 Dock 角标，不支持菜单栏/托盘图标角标。
- **iOS**：后续实现时，iOS 16+ 可能需要通知权限才能显示角标。
- **Android**：各厂商 Launcher 差异大，后续需适配 ShortcutBadger 等方案。

## 依赖概览

| 组件 | 用途 |
|------|------|
| `flutter_rust_bridge` 2.12.0 | Dart ↔ Rust 绑定 |
| `windows` 0.62 | Windows COM / GDI / Shell |
| `objc2` + `objc2-app-kit` | macOS AppKit |
| `objc2-ui-kit` | iOS UIKit（预留） |
| `zbus` 5.x | Linux D-Bus（预留） |
| `jni` 0.22 | Android JNI（预留） |
| Cargokit | Gradle / CocoaPods / CMake 构建 Rust |

## License

见仓库根目录 LICENSE 文件（如有）。
