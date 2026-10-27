import 'package:flutter/services.dart';
import 'package:material_ui/material_ui.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:xue_hua_app_badge/xue_hua_app_badge.dart';
import 'package:xue_hua_app_badge_example/main.dart' as app;

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('badge API returns expected results', (tester) async {
    expect(await XueHuaAppBadge.instance.isSupported(), isTrue);

    // Permission may be false on a fresh simulator; requestAuthorization can
    // present a system alert that blocks integration_test, so bound the wait.
    final bool grantedBefore =
        await XueHuaAppBadge.instance.isPermissionGranted();
    final bool requested = await XueHuaAppBadge.instance
        .requestPermission()
        .timeout(const Duration(seconds: 8), onTimeout: () => false);
    final bool grantedAfter =
        await XueHuaAppBadge.instance.isPermissionGranted();
    expect(grantedBefore, isA<bool>());
    expect(requested, isA<bool>());
    expect(grantedAfter, isA<bool>());

    await expectLater(
      XueHuaAppBadge.instance.set(-1),
      throwsA(
        isA<ArgumentError>().having(
          (ArgumentError error) => error.message,
          'message',
          'Badge count must be >= 0',
        ),
      ),
    );

    Future<void> runBadgeCalls() async {
      await XueHuaAppBadge.instance.set(0);
      await XueHuaAppBadge.instance.set(4);
      await XueHuaAppBadge.instance.set(4, windowHandle: 0);
      await XueHuaAppBadge.instance.remove();
      await XueHuaAppBadge.instance.remove(windowHandle: 0);
      for (var count = 0; count < 5; count++) {
        await XueHuaAppBadge.instance.set(count);
      }
      await XueHuaAppBadge.instance.remove();
    }

    try {
      await runBadgeCalls();
    } on PlatformException catch (error) {
      // Without notification/badge authorization, native setBadge may fail.
      expect(
        grantedAfter,
        isFalse,
        reason: 'setBadge failed while permission reported granted: $error',
      );
    }
  });

  testWidgets('example controls set and clear the badge', (tester) async {
    app.main();
    await tester.pumpAndSettle();

    expect(find.byKey(const Key('badge_count')), findsOneWidget);
    expect(find.text('Badge count: 0'), findsOneWidget);
    expect(find.byKey(const Key('badge_error')), findsNothing);
    expect(find.byKey(const Key('badge_permission_hint')), findsNothing);

    expect(
      tester
          .widget<FilledButton>(find.byKey(const Key('badge_decrement')))
          .onPressed,
      isNull,
    );

    await tester.tap(find.byKey(const Key('badge_increment')));
    await tester.pumpAndSettle();
    expect(find.text('Badge count: 1'), findsOneWidget);
    expect(find.byKey(const Key('badge_error')), findsNothing);

    await tester.tap(find.byKey(const Key('badge_increment')));
    await tester.pumpAndSettle();
    expect(find.text('Badge count: 2'), findsOneWidget);

    await tester.tap(find.byKey(const Key('badge_decrement')));
    await tester.pumpAndSettle();
    expect(find.text('Badge count: 1'), findsOneWidget);

    await tester.tap(find.byKey(const Key('badge_clear')));
    await tester.pumpAndSettle();
    expect(find.text('Badge count: 0'), findsOneWidget);
    expect(find.byKey(const Key('badge_error')), findsNothing);
    expect(
      tester
          .widget<FilledButton>(find.byKey(const Key('badge_decrement')))
          .onPressed,
      isNull,
    );
  });
}
