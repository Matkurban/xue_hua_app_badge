# Changelog

## 2.0.1

- The Gradle tool version has been downgraded to 8.13.2.

## 2.0.0

* **Complete Architecture Refactoring**: Removed `flutter_rust_bridge`, `ffi`, `ffigen`, CargoKit, and Rust core dependencies. Replaced with standard platform `MethodChannel` native implementations across Android, iOS, macOS, Windows, and Linux.
* **Dart API Refactoring**:
  * Converted `XueHuaAppBadge` into a singleton class accessible via `XueHuaAppBadge.instance` or `XueHuaAppBadge()`.
  * Removed redundant `XueHuaAppBadge.initialize()` method.
* **Android**:
  * Updated package name and namespace to `com.kurban.xue_hua_app_badge`.
  * Migrated Gradle build configuration scripts to Kotlin DSL (`build.gradle.kts` and `settings.gradle.kts`).
* **iOS & macOS**:
  * Added **Swift Package Manager (SPM)** support (`Package.swift`) according to official Flutter SPM plugin guidelines while maintaining CocoaPods podspec compatibility.
  * Implemented pure Swift plugins handling `UNUserNotificationCenter`, `UIApplication`, and `NSApp.dockTile`.
* **Windows & Linux**:
  * Implemented pure C++ Win32 `ITaskbarList3::SetOverlayIcon` for Windows.
  * Implemented pure C++ GTK / GDBus Unity LauncherEntry DBus signals for Linux.

## 1.0.9

* Android: Fix JNI `ClassNotFoundException` linkage error when calling `PermissionHelper` or `BadgeHelper` static methods from Rust background threads. Cache global class references (`Global<JClass>`) during `initAndroid` on the Java main thread to bypass system `BootClassLoader` restrictions on attached native threads.
* Android: Add `@Keep` annotations to `PermissionHelper`, `BadgeHelper`, and `XueHuaAppBadgePlugin`, and add `consumer-rules.pro` to prevent R8 / ProGuard minification from stripping or obfuscating JNI target classes in release builds.

## 1.0.8

* Android: Fix main-thread deadlock that froze the app for 30s and triggered an ANR when `requestPermission()` was called on Android 13+ without `POST_NOTIFICATIONS` granted. The permission result is dispatched on the main thread, so waiting for it there could never complete.
* **API change**: `XueHuaAppBadge.requestPermission()` now returns `Future<bool>` instead of `bool`, so the wait happens off the main thread. Add `await` at call sites. `set()` / `remove()` / `isPermissionGranted()` are unchanged.
* Android: `PermissionHelper.requestBadgePermission` now refuses to block when called on the main looper, and always raises the dialog via `runOnUiThread`.

## 1.0.7

* macOS/iOS: Fix Cargokit `build_pod.sh` failure caused by CRLF line endings in the 1.0.6 pub.dev package (`set: -: invalid option`)
* Add `.gitattributes` so shell scripts stay LF on publish

## 1.0.6

* Windows: Fix Rust build against `windows` crate 0.62 (import relocation, GDI signatures, `AgileReference` for COM, stable `Once` init)
* Windows: Render DPI-aware overlay badges with 2x supersampling and soft-edge alpha
* Windows: Center badge digits by ink bounding box (fix top-left offset / blurry glyphs)

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
