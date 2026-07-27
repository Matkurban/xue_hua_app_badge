import 'package:integration_test/integration_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:xue_hua_app_badge/xue_hua_app_badge.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  test('badge set and remove', () async {
    expect(() async => await XueHuaAppBadge.instance.set(0), returnsNormally);
    expect(() async => await XueHuaAppBadge.instance.remove(), returnsNormally);
  });
}
