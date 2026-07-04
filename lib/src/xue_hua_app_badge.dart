import '../src/rust/api/badge.dart';
import '../src/rust/frb_generated.dart';
import 'dart:developer';

/// Unified Dart wrapper for the Rust badge API.
///
/// Call [initialize] once at app startup before using any other methods.
class XueHuaAppBadge {
  const XueHuaAppBadge._();

  /// Initializes the Rust FFI bridge. Must be called before any badge API.
  static Future<void> initialize() async {
    try {
      if (!RustLib.instance.initialized) {
        await RustLib.init();
      }
    } catch (e, s) {
      log(e.toString(), error: e, stackTrace: s, name: 'XueHuaAppBadge.initialize');
    }
  }

  static void set(int count, {int? windowHandle}) {
    setBadge(count: count, windowHandle: windowHandle);
  }

  static void remove({int? windowHandle}) {
    removeBadge(windowHandle: windowHandle);
  }

  static bool requestPermission() => requestBadgePermission();

  static bool isPermissionGranted() => isBadgePermissionGranted();
}
