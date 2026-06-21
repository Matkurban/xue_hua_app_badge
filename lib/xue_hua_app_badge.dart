library;

import 'src/platform_badge.dart';

export 'src/platform_badge.dart' show RustLib, initBadgePlugin, usesRustBadge;

/// Unified cross-platform badge API.
///
/// - Android / iOS / macOS: native MethodChannel
/// - Windows / Linux: flutter_rust_bridge (Rust)
class XueHuaAppBadge {
  const XueHuaAppBadge._();

  static Future<void> set(int count, {int? windowHandle}) {
    if (count < 0) {
      throw ArgumentError.value(count, 'count', 'Badge count must be >= 0');
    }
    return platformSetBadge(count, windowHandle: windowHandle);
  }

  static Future<void> remove({int? windowHandle}) {
    return platformRemoveBadge(windowHandle: windowHandle);
  }

  static Future<bool> requestPermission() {
    return platformRequestPermission();
  }

  static Future<bool> isPermissionGranted() {
    return platformIsPermissionGranted();
  }
}
