library;

import 'src/rust/api/badge.dart';
import 'src/rust/frb_generated.dart';

export 'src/rust/api/badge.dart'
    show
        isBadgePermissionGranted,
        removeBadge,
        requestBadgePermission,
        setBadge;
export 'src/rust/api/simple.dart' show greet;

/// Unified Dart wrapper for the Rust badge API.
///
/// Call [initialize] once at app startup before using any other methods.
class XueHuaAppBadge {
  const XueHuaAppBadge._();

  /// Initializes the Rust FFI bridge. Must be called before any badge API.
  static Future<void> initialize() => RustLib.init();

  static void set(int count, {int? windowHandle}) {
    setBadge(count: count, windowHandle: windowHandle);
  }

  static void remove({int? windowHandle}) {
    removeBadge(windowHandle: windowHandle);
  }

  static bool requestPermission() => requestBadgePermission();

  static bool isPermissionGranted() => isBadgePermissionGranted();
}
