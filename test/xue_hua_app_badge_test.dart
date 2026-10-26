import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:xue_hua_app_badge/xue_hua_app_badge.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  const MethodChannel channel = MethodChannel('xue_hua_app_badge');
  final List<MethodCall> log = <MethodCall>[];

  setUp(() {
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(channel, (MethodCall methodCall) async {
          log.add(methodCall);
          switch (methodCall.method) {
            case 'isSupported':
              return true;
            case 'setBadge':
              return true;
            case 'removeBadge':
              return true;
            case 'requestPermission':
              return true;
            case 'isPermissionGranted':
              return true;
            default:
              return null;
          }
        });
    log.clear();
  });

  tearDown(() {
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(channel, null);
  });

  test('singleton instance equality', () {
    expect(XueHuaAppBadge.instance, same(XueHuaAppBadge()));
  });

  test('isSupported', () async {
    final bool supported = await XueHuaAppBadge.instance.isSupported();
    expect(supported, isTrue);
    expect(log, <Matcher>[isMethodCall('isSupported', arguments: null)]);
  });

  test('set badge count', () async {
    await XueHuaAppBadge.instance.set(5);
    expect(log, <Matcher>[
      isMethodCall('setBadge', arguments: <String, dynamic>{'count': 5}),
    ]);
  });

  test(
    'set forwards windowHandle and accepts a bool platform result',
    () async {
      await XueHuaAppBadge.instance.set(2, windowHandle: 42);
      expect(log, <Matcher>[
        isMethodCall(
          'setBadge',
          arguments: <String, dynamic>{'count': 2, 'windowHandle': 42},
        ),
      ]);
    },
  );

  test('set rejects a negative count before calling the platform', () async {
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
    expect(log, isEmpty);
  });

  test('set zero still calls setBadge', () async {
    await XueHuaAppBadge.instance.set(0);
    expect(log, <Matcher>[
      isMethodCall('setBadge', arguments: <String, dynamic>{'count': 0}),
    ]);
  });

  test('remove badge', () async {
    await XueHuaAppBadge.instance.remove();
    expect(log, <Matcher>[isMethodCall('removeBadge', arguments: null)]);
  });

  test('remove forwards windowHandle', () async {
    await XueHuaAppBadge.instance.remove(windowHandle: 7);
    expect(log, <Matcher>[
      isMethodCall(
        'removeBadge',
        arguments: <String, dynamic>{'windowHandle': 7},
      ),
    ]);
  });

  test('request permission', () async {
    final bool granted = await XueHuaAppBadge.instance.requestPermission();
    expect(granted, isTrue);
    expect(log, <Matcher>[isMethodCall('requestPermission', arguments: null)]);
  });

  test('is permission granted', () async {
    final bool granted = await XueHuaAppBadge.instance.isPermissionGranted();
    expect(granted, isTrue);
    expect(log, <Matcher>[
      isMethodCall('isPermissionGranted', arguments: null),
    ]);
  });
}
