import 'dart:io' show Platform;

import 'package:flutter/foundation.dart';

import 'method_channel_badge.dart';
import 'rust/frb_generated.dart';
import 'rust/api/badge.dart' as rust_badge;

export 'rust/frb_generated.dart' show RustLib;

bool get usesRustBadge {
  if (kIsWeb) {
    return false;
  }
  return Platform.isWindows || Platform.isLinux;
}

bool get usesMethodChannelBadge {
  if (kIsWeb) {
    return false;
  }
  return Platform.isAndroid || Platform.isIOS || Platform.isMacOS;
}

/// Initializes platform backends. Call once before using [XueHuaAppBadge].
Future<void> initBadgePlugin() async {
  if (usesRustBadge) {
    await RustLib.init();
  }
}

Future<void> platformSetBadge(int count, {int? windowHandle}) async {
  if (usesRustBadge) {
    rust_badge.setBadge(count: count, windowHandle: windowHandle);
    return;
  }
  if (usesMethodChannelBadge) {
    await MethodChannelBadge.set(count);
    return;
  }
  throw UnsupportedError('Badge is not supported on this platform');
}

Future<void> platformRemoveBadge({int? windowHandle}) async {
  if (usesRustBadge) {
    rust_badge.removeBadge(windowHandle: windowHandle);
    return;
  }
  if (usesMethodChannelBadge) {
    await MethodChannelBadge.remove();
    return;
  }
  throw UnsupportedError('Badge is not supported on this platform');
}

Future<bool> platformRequestPermission() async {
  if (usesRustBadge) {
    return rust_badge.requestBadgePermission();
  }
  if (usesMethodChannelBadge) {
    return MethodChannelBadge.requestPermission();
  }
  return true;
}

Future<bool> platformIsPermissionGranted() async {
  if (usesRustBadge) {
    return rust_badge.isBadgePermissionGranted();
  }
  if (usesMethodChannelBadge) {
    return MethodChannelBadge.isPermissionGranted();
  }
  return true;
}
