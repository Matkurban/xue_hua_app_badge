---
name: xue-hua-app-badge-api
description: >-
  Use when writing, reviewing, or debugging Dart/Flutter code that sets,
  removes, or checks app icon badges with the xue_hua_app_badge package
  (XueHuaAppBadge.instance set/remove/requestPermission/isPermissionGranted/isSupported).
  Ensures the singleton API, permission-before-set on iOS 16+ and Android 13+,
  windowHandle usage on Windows, and platform-specific badge behavior.
---

# xue_hua_app_badge API

Authoritative instructions for `package:xue_hua_app_badge`. The Dart public surface is a single class: `XueHuaAppBadge`. Do not invent other Dart types or methods.

Read [references/dart-api.md](references/dart-api.md) before writing call sites. Read [references/platforms.md](references/platforms.md) for iOS/Android permissions, OEM launchers, Windows `windowHandle`, and `PlatformException` codes.

## Guidelines

* Always import `package:xue_hua_app_badge/xue_hua_app_badge.dart`.
* Always call instance methods on `XueHuaAppBadge.instance` (or `XueHuaAppBadge()`, which returns the same singleton). Never call `XueHuaAppBadge.set(...)` as a static method.
* On iOS 16+ and Android 13+, call `isPermissionGranted()` first. If it is `false`, call `requestPermission()`, then `set`. `set()` never shows a permission dialog.
* `set` and `remove` return `Future<void>`. Do not treat their result as a `bool`. Android native may return whether a launcher applied the badge; Dart discards that value.
* `count` must be `>= 0`. `set` throws `ArgumentError` for negatives. Clear the badge with `remove()` or `set(0)`.
* `remove()` calls `set(0)` and therefore invokes MethodChannel `'setBadge'` with `{count: 0}`. Do not invent a Dart `removeBadge()` method.
* Pass `windowHandle` only on Windows (HWND as `int`). Other platforms ignore it. Omit it unless targeting a specific Windows window; the plugin then uses the Flutter view's root HWND.
* Do not call `initialize()`, `greet()`, or any `PlatformInterface` subclass. Those are not part of this package's Dart API.
* Do not talk to MethodChannel `'xue_hua_app_badge'` from app code. Use `XueHuaAppBadge` only.
* This plugin registers Android, iOS, macOS, Windows, and Linux. Do not assume Web or a plain Dart VM.

## Examples

### Permission check, set, and clear

```dart
import 'package:xue_hua_app_badge/xue_hua_app_badge.dart';

Future<void> updateBadge(int count) async {
  if (!await XueHuaAppBadge.instance.isPermissionGranted()) {
    final granted = await XueHuaAppBadge.instance.requestPermission();
    if (!granted) {
      return;
    }
  }
  await XueHuaAppBadge.instance.set(count);
}

Future<void> clearBadge() async {
  await XueHuaAppBadge.instance.remove();
}
```

### Factory equals singleton

```dart
final a = XueHuaAppBadge.instance;
final b = XueHuaAppBadge();
// identical(a, b) == true
await a.set(5);
```
