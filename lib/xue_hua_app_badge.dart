library;

import 'src/rust/api/badge.dart';

export 'src/rust/api/badge.dart' show removeBadge, setBadge;
export 'src/rust/api/simple.dart' show greet;
export 'src/rust/frb_generated.dart' show RustLib;

/// Unified Dart wrapper for the Rust badge API.
class XueHuaAppBadge {
  const XueHuaAppBadge._();

  static void set(int count, {int? windowHandle}) {
    setBadge(count: count, windowHandle: windowHandle);
  }

  static void remove({int? windowHandle}) {
    removeBadge(windowHandle: windowHandle);
  }
}
