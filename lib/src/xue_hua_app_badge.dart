import 'package:flutter/services.dart';

/// Unified Dart wrapper for the platform badge API via MethodChannel.
class XueHuaAppBadge {
  XueHuaAppBadge._();

  /// Singleton instance of [XueHuaAppBadge].
  static final XueHuaAppBadge instance = XueHuaAppBadge._();

  /// Factory constructor returning the singleton instance.
  factory XueHuaAppBadge() => instance;

  final MethodChannel _channel = const MethodChannel('xue_hua_app_badge');

  /// Returns whether badge functionality is supported on the current platform.
  Future<bool> isSupported() async {
    final bool? result = await _channel.invokeMethod<bool>('isSupported');
    return result ?? true;
  }

  /// Sets the application icon badge count.
  Future<void> set(int count, {int? windowHandle}) async {
    if (count < 0) {
      throw ArgumentError('Badge count must be >= 0');
    }
    final Map<String, dynamic> arguments = <String, dynamic>{'count': count};
    if (windowHandle != null) {
      arguments['windowHandle'] = windowHandle;
    }
    await _channel.invokeMethod<void>('setBadge', arguments);
  }

  /// Removes the application icon badge count.
  Future<void> remove({int? windowHandle}) async {
    await set(0, windowHandle: windowHandle);
  }

  /// Shows the platform permission prompt and resolves with the user's answer.
  Future<bool> requestPermission() async {
    final bool? result = await _channel.invokeMethod<bool>('requestPermission');
    return result ?? true;
  }

  /// Checks if badge permission has been granted.
  Future<bool> isPermissionGranted() async {
    final bool? result = await _channel.invokeMethod<bool>(
      'isPermissionGranted',
    );
    return result ?? true;
  }
}
