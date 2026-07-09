## 1.0.5

* Android 使用 current_thread FRB handler，避免多插件并存时 pthread_key 耗尽。

## 1.0.4

* Optimized initialization logic

## 1.0.3

* iOS: Fix main-thread Condvar deadlock on iOS 16+; add `ios_async` RunLoop bridge module
* iOS: `isPermissionGranted()` now checks `badgeSetting`, not only `authorizationStatus`
* Android: Fix concurrent `requestPermission()` race (single-flight + requestId)
* Android: Retain Activity reference during configuration changes to avoid interrupting permission dialogs
* Android: Use `Once` for JNI initialization; log failures to logcat
* Rust: Add `initialize()` guard for badge APIs; unit tests for `format_badge_label` / `badge_number`
* Dart: Remove public `greet()` export; add badge integration tests
* Docs: Add `CONTEXT.md` domain glossary

## 1.0.2

* Android: Migrate Context/JavaVM to `ndk-context 0.1.1`, simplifying Rust context management
* Android: Fix compilation errors from `jni 0.22` API incompatibility
* Docs: Add `README.zh-CN.md` with cross-links between English and Chinese READMEs
* Docs: Document Rust / Cargokit build requirements and common errors per platform

## 1.0.1

* First stable release
* Add `XueHuaAppBadge.initialize()` as the unified initialization entry point
* Stop exporting `RustLib` publicly; use the `XueHuaAppBadge` public API instead
* Support badge and permission APIs on Android, iOS, macOS, Windows, and Linux
* Fix README installation and quick-start examples
