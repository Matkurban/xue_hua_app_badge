import 'package:integration_test/integration_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:xue_hua_app_badge/xue_hua_app_badge.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  setUpAll(() async {
    await XueHuaAppBadge.initialize();
  });

  test('badge API smoke test', () {
    expect(() => XueHuaAppBadge.set(0), returnsNormally);
    expect(() => XueHuaAppBadge.set(1), returnsNormally);
    expect(() => XueHuaAppBadge.remove(), returnsNormally);
    expect(XueHuaAppBadge.isPermissionGranted(), isA<bool>());
  });

  test('requestPermission returns bool', () {
    expect(XueHuaAppBadge.requestPermission(), isA<bool>());
  });
}
