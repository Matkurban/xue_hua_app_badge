## 1.0.0

首个稳定版本，提供跨平台应用角标（Badge）统一 API。

### 功能

- 统一 API：`XueHuaAppBadge.set()` / `remove()` / `requestPermission()` / `isPermissionGranted()`
- **Android**：ShortcutBadger + NotificationChannel 回退，Kotlin MethodChannel
- **iOS**：`UNUserNotificationCenter` / `UIApplication`，Swift MethodChannel
- **macOS**：`NSApplication.dockTile.badgeLabel`，Swift MethodChannel
- **Windows**：`ITaskbarList3::SetOverlayIcon`，Rust (flutter_rust_bridge)
- **Linux**：Unity LauncherEntry D-Bus，Rust (flutter_rust_bridge)
- 数字超过 99 时：macOS 显示 `99+`，其他平台显示 `99`

### 架构与构建

- 采用混合架构：Android / iOS / macOS 走原生 MethodChannel，Windows / Linux 走 Rust FFI
- **iOS / macOS**：添加 Swift Package Manager（SPM）支持，同时保留 CocoaPods 兼容
- **Android**：迁移至 Built-in Kotlin，移除 `kotlin-android` KGP 依赖
- 最低环境要求：**Flutter >= 3.44.0**，**Dart ^3.12.0**
