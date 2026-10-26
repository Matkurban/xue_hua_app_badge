# xue_hua_app_badge example

Sample app for the `xue_hua_app_badge` plugin.

The example requires Flutter >=3.47 (Dart >=3.13) because it uses `package:material_ui`. Flutter 3.50 deprecates `package:flutter/material.dart`, and `material_ui` 1.8 is the replacement that also analyzes cleanly on current stable. The plugin package itself still allows Flutter >=3.44 and does not import Material.

## macOS project

`example/macos` is Swift Package Manager only. It has no `Podfile`. The Runner deployment target is macOS 12.0 (`MACOSX_DEPLOYMENT_TARGET` on the Debug, Profile, and Release project configurations). `Runner/Info.plist` still sets `LSMinimumSystemVersion` from `$(MACOSX_DEPLOYMENT_TARGET)`.

Adding a `Podfile` back makes Flutter print that every macOS plugin is a Swift package while the project still has CocoaPods integration. Follow [Swift Package Manager for app developers](https://docs.flutter.dev/packages-and-plugins/swift-package-manager/for-app-developers) if that warning returns: `pod deintegrate` in `macos/`, delete the `Podfile`, and remove any `Pods/Target Support Files` includes from `Flutter/Flutter-Debug.xcconfig` and `Flutter/Flutter-Release.xcconfig`.

The plugin package still ships both `macos/xue_hua_app_badge/Package.swift` and `macos/xue_hua_app_badge.podspec`, so apps that have not left CocoaPods can keep using the podspec.

## Integration tests

From this directory, run the desktop integration test with the Flutter test runner:

```bash
flutter test integration_test -d windows
# or: -d linux / -d macos
```

`flutter drive --driver=test_driver/integration_test.dart --target=integration_test/badge_test.dart` can print:

```text
Warning: integration_test plugin was not detected.
```

That message comes from `IntegrationTestWidgetsFlutterBinding` when `allTestsFinished` is sent on the `plugins.flutter.io/integration_test` channel and no native handler answers. The `integration_test` native plugin is part of Android instrumentation and iOS XCTest only. `flutter drive` on Windows, Linux, and macOS starts the app through the VM service, so desktop has no native plugin to register. The example already depends on `integration_test` from the Flutter SDK. The warning is not a missing dependency, and the tests can still pass.

`flutter test integration_test -d <device>` collects results in the test harness and does not print that warning. Use it for local runs and CI.

`Failed to foreground app; open returned 1` can appear while `flutter test integration_test -d macos` or `flutter drive` launches the example. Flutter's macOS device code calls `/usr/bin/open` to raise the window, and that call can return 1 during launch (LaunchServices `procNotFound`, [flutter/flutter#176850](https://github.com/flutter/flutter/issues/176850)). The example and its tests do not invoke `open`. When the tests still pass, treat the line as a Flutter desktop quirk.
