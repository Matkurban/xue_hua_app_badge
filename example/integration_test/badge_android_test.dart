import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:xue_hua_app_badge/xue_hua_app_badge.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  setUpAll(() async {
    await initBadgePlugin();
  });

  testWidgets('isPermissionGranted returns without error', (tester) async {
    await expectLater(XueHuaAppBadge.isPermissionGranted(), completes);
  });

  testWidgets('requestPermission returns without error', (tester) async {
    await expectLater(XueHuaAppBadge.requestPermission(), completes);
  });

  testWidgets('set(3) and remove() succeed', (tester) async {
    await expectLater(XueHuaAppBadge.set(3), completes);
    await expectLater(XueHuaAppBadge.remove(), completes);
  });

  testWidgets('set(0) clears badge', (tester) async {
    await XueHuaAppBadge.set(5);
    await expectLater(XueHuaAppBadge.set(0), completes);
  });
}
