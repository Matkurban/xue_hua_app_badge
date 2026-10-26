// Prefer `flutter test integration_test -d <device>` from example/.
// `flutter drive` using this VM-service driver prints
// "Warning: integration_test plugin was not detected" on desktop, including
// macOS. The integration_test native plugin is registered for Android
// instrumentation and iOS XCTest only, so the desktop driver has nothing to
// detect. The warning is not a missing example dependency, and `flutter test`
// does not print it.
import 'package:integration_test/integration_test_driver.dart';

Future<void> main() => integrationDriver();
