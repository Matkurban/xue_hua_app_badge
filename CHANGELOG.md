## 1.0.2

* Android：Context/JavaVM 迁移至 `ndk-context 0.1.1`，简化 Rust 上下文管理
* Android：修复 `jni 0.22` API 不兼容导致的编译错误
* 文档：新增 `README.zh-CN.md`，中英文 README 互相导航
* 文档：补充 Rust / Cargokit 各平台编译条件与常见错误说明

## 1.0.1

* 首个稳定发布
* 新增 `XueHuaAppBadge.initialize()` 作为统一初始化入口
* 不再对外导出 `RustLib`；请使用 `XueHuaAppBadge` 公开 API
* 支持 Android、iOS、macOS、Windows、Linux 五端角标与权限 API
* 修正 README 安装与快速开始示例
