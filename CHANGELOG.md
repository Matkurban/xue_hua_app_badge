# Changelog

## 2.1.9

- Example: Make the badge `integration_test` simulator-safe. Do not hard-require notification permission on a fresh simulator, bound `requestPermission` with a timeout so a system alert cannot hang the suite, and tolerate `PlatformException` from `setBadge` when authorization is denied. No plugin API change.

## 2.1.8

- Publishing: 2.1.7 was 29,973,064 bytes (about 28.6MB). 2.1.6 was 316,716 bytes (about 309KB). `.pubignore` contained only `rust/Cargo.lock`, and when that file exists pub uses it instead of `.gitignore` in the same directory, so the root ignore rules never applied. The archive included `build/test_cache` (two ~45MB `*.cache.dill.track.dill` files), `build/unit_test_assets`, and `build/native_assets`. `.pubignore` now follows `.gitignore` and the usual Flutter plugin publish excludes: `/build/`, `**/build/`, `.dart_tool/`, `**/test_cache/`, `*.dill`, coverage, IDE files, `example/pubspec.lock`, Android/iOS/desktop generated output, and `rust/Cargo.lock`. No API change.
- Example: Remove the committed Gradle problems report at `example/android/build/reports/problems/problems-report.html` and ignore `example/android/build/`. It is a local build artifact.

## 2.1.7

- Example macOS: Remove CocoaPods from the example runner so the project is Swift Package Manager only. Deleted `example/macos/Podfile` and `Podfile.lock`, dropped the `Pods/Pods.xcodeproj` reference from `Runner.xcworkspace`, and removed the `Pods-Runner` xcconfig includes from `Flutter-Debug.xcconfig` and `Flutter-Release.xcconfig`. Every macOS plugin in the example is already a Swift package, and the leftover CocoaPods integration made Flutter print the dual-integration warning on every macOS build.
- Example macOS: Set `MACOSX_DEPLOYMENT_TARGET` to 12.0 (Debug, Profile, and Release). That is the minimum Flutter 3.47 writes for macOS. `LSMinimumSystemVersion` still follows `$(MACOSX_DEPLOYMENT_TARGET)`. The plugin's own macOS deployment target is unchanged.

## 2.1.6

- Dart: `remove()` now calls MethodChannel `removeBadge` (and forwards `windowHandle` when it is set) instead of `set(0)`. `set(0)` still clears through `setBadge`. Android, iOS, macOS, Windows, and Linux already clear the badge in `removeBadge` the same way as a zero count, so the visible result is unchanged.
- Example: Rename the integration test to platform-neutral wording. It runs on Windows and Linux, not only Linux.
- Example: The Clear control calls `remove()`.
- Example: Import `package:material_ui/material_ui.dart` instead of `package:flutter/material.dart`. Beta 3.50 deprecates the in-framework library (`deprecated_member_use`, which fails `flutter analyze`). `material_ui` 1.8 is the supported replacement on current stable (Flutter 3.47) and on that beta, and it re-exports `package:flutter/widgets.dart`. The example SDK floor is therefore Dart `^3.13.0` and Flutter `>=3.47.0`. The plugin library itself still supports Flutter `>=3.44.0` and does not depend on `material_ui`.
- Docs: On desktop, run `flutter test integration_test -d <device>` from `example/`. `flutter drive` prints `Warning: integration_test plugin was not detected` because the integration_test native plugin exists for Android and iOS instrumentation only. The desktop VM-service driver has nothing to register, and the warning is not a missing example dependency. `flutter test` does not print it.

## 2.1.5

- Linux: Address Unity LauncherEntry updates at `application://<desktop-file-id>`. The id is the basename of `GIO_LAUNCHED_DESKTOP_FILE`, otherwise `GAPPLICATION_ID`, otherwise `flutter`, with `.desktop` appended when it is missing. Flutter Linux apps set `GAPPLICATION_ID` to the application id, so the previous URI did not match the desktop file.
- Linux: Publish `count` `0` together with `count-visible` false when clearing a badge, and ignore a null `setBadge` argument map.
- Example: Drive the sample UI from one `integration_test` target (plus the integration_test driver) and assert `isSupported`, `set`, `remove`, permission results, and the negative-count error.

## 2.1.4

- Linux: Fix build failure caused by the nonexistent `g_autorelease` macro; use GLib `g_autoptr(...)` for automatic cleanup instead.

## 2.1.3

- Android: Upgrade the build toolchain to Android Gradle Plugin 9.1.0, Gradle 9.3.1, and Kotlin 2.4.0.
- Android: Upgrade `androidx.core:core-ktx` to 1.18.0, `androidx.test:core` to 1.7.0, and Robolectric to 4.17 so they stay compatible with this toolchain and `compileSdk` 36.
- Android: Pin the Robolectric default test SDK to API 28 so unit tests keep running on JDK 17.

## 2.1.2

- Ship Agent Skills under `skills/` so coding agents can install `XueHuaAppBadge` usage rules with `dart run skills@ get`.

## 2.1.1

- streamline URI initialization and permission checks in badge handling classes

## 2.1.0

- Android: Honor, Huawei, Xiaomi, OPPO, vivo, and Meizu now use each OEM's official badge API instead of ShortcutBadger for those devices.
- Android: Keep ShortcutBadger for other launchers and the silent Android O notification-dot fallback.
- Android: Honor/Huawei numeric badges are not cleared by opening the app or dismissing notifications; call `remove()` or `set(0)`. Xiaomi badges follow notification count (progress and ongoing notifications are ignored). vivo requires the per-app desktop-badge setting. OPPO numeric badges are whitelist-gated.

## 2.0.1

- The Gradle tool version has been downgraded to 8.13.2.

## 2.0.0

- **Complete Architecture Refactoring**: Removed `flutter_rust_bridge`, `ffi`, `ffigen`, CargoKit, and Rust core dependencies. Replaced with standard platform `MethodChannel` native implementations across Android, iOS, macOS, Windows, and Linux.
- **Dart API Refactoring**:
  - Converted `XueHuaAppBadge` into a singleton class accessible via `XueHuaAppBadge.instance` or `XueHuaAppBadge()`.
  - Removed redundant `XueHuaAppBadge.initialize()` method.
- **Android**:
  - Updated package name and namespace to `com.kurban.xue_hua_app_badge`.
  - Migrated Gradle build configuration scripts to Kotlin DSL (`build.gradle.kts` and `settings.gradle.kts`).
- **iOS & macOS**:
  - Added **Swift Package Manager (SPM)** support (`Package.swift`) according to official Flutter SPM plugin guidelines while maintaining CocoaPods podspec compatibility.
  - Implemented pure Swift plugins handling `UNUserNotificationCenter`, `UIApplication`, and `NSApp.dockTile`.
- **Windows & Linux**:
  - Implemented pure C++ Win32 `ITaskbarList3::SetOverlayIcon` for Windows.
  - Implemented pure C++ GTK / GDBus Unity LauncherEntry DBus signals for Linux.

## 1.0.9

- Android: Fix JNI `ClassNotFoundException` linkage error when calling `PermissionHelper` or `BadgeHelper` static methods from Rust background threads. Cache global class references (`Global<JClass>`) during `initAndroid` on the Java main thread to bypass system `BootClassLoader` restrictions on attached native threads.
- Android: Add `@Keep` annotations to `PermissionHelper`, `BadgeHelper`, and `XueHuaAppBadgePlugin`, and add `consumer-rules.pro` to prevent R8 / ProGuard minification from stripping or obfuscating JNI target classes in release builds.

## 1.0.8

- Android: Fix main-thread deadlock that froze the app for 30s and triggered an ANR when `requestPermission()` was called on Android 13+ without `POST_NOTIFICATIONS` granted. The permission result is dispatched on the main thread, so waiting for it there could never complete.
- **API change**: `XueHuaAppBadge.requestPermission()` now returns `Future<bool>` instead of `bool`, so the wait happens off the main thread. Add `await` at call sites. `set()` / `remove()` / `isPermissionGranted()` are unchanged.
- Android: `PermissionHelper.requestBadgePermission` now refuses to block when called on the main looper, and always raises the dialog via `runOnUiThread`.

## 1.0.7

- macOS/iOS: Fix Cargokit `build_pod.sh` failure caused by CRLF line endings in the 1.0.6 pub.dev package (`set: -: invalid option`)
- Add `.gitattributes` so shell scripts stay LF on publish

## 1.0.6

- Windows: Fix Rust build against `windows` crate 0.62 (import relocation, GDI signatures, `AgileReference` for COM, stable `Once` init)
- Windows: Render DPI-aware overlay badges with 2x supersampling and soft-edge alpha
- Windows: Center badge digits by ink bounding box (fix top-left offset / blurry glyphs)

## 1.0.5

- Android 使用 current_thread FRB handler，避免多插件并存时 pthread_key 耗尽。

## 1.0.4

- Optimized initialization logic

## 1.0.3

- iOS: Fix main-thread Condvar deadlock on iOS 16+; add `ios_async` RunLoop bridge module
- iOS: `isPermissionGranted()` now checks `badgeSetting`, not only `authorizationStatus`
- Android: Fix concurrent `requestPermission()` race (single-flight + requestId)
- Android: Retain Activity reference during configuration changes to avoid interrupting permission dialogs
- Android: Use `Once` for JNI initialization; log failures to logcat
- Rust: Add `initialize()` guard for badge APIs; unit tests for `format_badge_label` / `badge_number`
- Dart: Remove public `greet()` export; add badge integration tests
- Docs: Add `CONTEXT.md` domain glossary

## 1.0.2

- Android: Migrate Context/JavaVM to `ndk-context 0.1.1`, simplifying Rust context management
- Android: Fix compilation errors from `jni 0.22` API incompatibility
- Docs: Add `README.zh-CN.md` with cross-links between English and Chinese READMEs
- Docs: Document Rust / Cargokit build requirements and common errors per platform

## 1.0.1

- First stable release
- Add `XueHuaAppBadge.initialize()` as the unified initialization entry point
- Stop exporting `RustLib` publicly; use the `XueHuaAppBadge` public API instead
- Support badge and permission APIs on Android, iOS, macOS, Windows, and Linux
- Fix README installation and quick-start examples
