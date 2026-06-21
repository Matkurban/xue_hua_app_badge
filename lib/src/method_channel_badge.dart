import 'package:flutter/services.dart';

/// MethodChannel bridge for Android, iOS, and macOS native badge APIs.
class MethodChannelBadge {
  const MethodChannelBadge._();

  static const MethodChannel _channel = MethodChannel('xue_hua_app_badge');

  static Future<void> set(int count) async {
    await _channel.invokeMethod<void>('setBadge', {'count': count});
  }

  static Future<void> remove() async {
    await _channel.invokeMethod<void>('removeBadge');
  }

  static Future<bool> requestPermission() async {
    final granted = await _channel.invokeMethod<bool>('requestPermission');
    return granted ?? false;
  }

  static Future<bool> isPermissionGranted() async {
    final granted = await _channel.invokeMethod<bool>('isPermissionGranted');
    return granted ?? false;
  }
}
