# xue_hua_app_badge example

Sample app for the `xue_hua_app_badge` plugin.

The example requires Flutter >=3.47 (Dart >=3.13) because it uses `package:material_ui`. Flutter 3.50 deprecates `package:flutter/material.dart`, and `material_ui` 1.8 is the replacement that also analyzes cleanly on current stable. The plugin package itself still allows Flutter >=3.44 and does not import Material.

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

That message comes from `IntegrationTestWidgetsFlutterBinding` when `allTestsFinished` is sent on the `plugins.flutter.io/integration_test` channel and no native handler answers. The `integration_test` native plugin is part of Android instrumentation and XCTest only. Windows and Linux have no native plugin to register, and `flutter drive` starts the app through the VM service. The example already depends on `integration_test` from the Flutter SDK. The warning is not a missing dependency, and the tests can still pass.

`flutter test integration_test -d <device>` collects results in the test harness and does not print that warning. Use it for local runs and CI.
