# XueHuaAppBadge Dart API

Source of truth: `lib/src/xue_hua_app_badge.dart`, re-exported by `lib/xue_hua_app_badge.dart`. Channel name: `'xue_hua_app_badge'`. Channel mappings below match `test/xue_hua_app_badge_test.dart`.

There are no other public Dart classes in this package.

## Class `XueHuaAppBadge`

Unified Dart wrapper for the platform badge API via `MethodChannel`.

The class is a singleton. App code must not construct additional instances.

## `XueHuaAppBadge._()`

**Signature:** `XueHuaAppBadge._()`

**Visibility:** private generative constructor.

**Purpose:** Prevents additional instances. The only instance is created as `instance`.

**Do not write:**

```dart
// Invalid: the generative constructor is private.
final badge = XueHuaAppBadge._();
```

## `XueHuaAppBadge.instance`

**Signature:** `static final XueHuaAppBadge instance`

**Purpose:** The singleton. Prefer this at every call site.

**Channel:** none (field access).

**Throws:** nothing.

**Correct:**

```dart
await XueHuaAppBadge.instance.set(3);
```

**Incorrect:**

```dart
// There is no static set.
await XueHuaAppBadge.set(3);
```

## `XueHuaAppBadge()` factory

**Signature:** `factory XueHuaAppBadge()`

**Returns:** the same object as `XueHuaAppBadge.instance`.

**Channel:** none.

**Throws:** nothing.

**Correct:**

```dart
final badge = XueHuaAppBadge();
assert(identical(badge, XueHuaAppBadge.instance));
await badge.remove();
```

## `isSupported`

**Signature:** `Future<bool> isSupported()`

**Parameters:** none.

**Returns:** `true` if the platform reports support; `true` when the channel returns `null` (`result ?? true`).

**Channel:** `'isSupported'`, arguments `null`.

**Throws:** `PlatformException` if the invocation fails. On Android that includes `NO_CONTEXT` when the plugin has no application context. When the native handler runs, Android, iOS, macOS, Windows, and Linux all return `true`.

**Correct:**

```dart
final supported = await XueHuaAppBadge.instance.isSupported();
```

**Incorrect:**

```dart
// Not a getter and not synchronous.
if (XueHuaAppBadge.instance.isSupported) {}
```

## `set`

**Signature:** `Future<void> set(int count, {int? windowHandle})`

**Parameters:**

- `count` (`int`, required): badge number. Must be `>= 0`. `0` clears the badge (same as `remove`).
- `windowHandle` (`int?`, optional, named): Windows HWND. Included in the channel map only when non-null. Ignored on non-Windows platforms.

**Returns:** `Future<void>`. Completing does not mean a launcher showed a numeric badge. Android native returns a `bool` applied flag; Dart invokes the method as `<void>` and discards it.

**Channel:** `'setBadge'`, arguments `{ 'count': count }` and, if provided, `'windowHandle': windowHandle`.

**Throws:**

- `ArgumentError` with message `Badge count must be >= 0` when `count < 0` (before any channel call).
- `PlatformException` when a native handler reports an error (see [platforms.md](platforms.md)).

**Correct:**

```dart
await XueHuaAppBadge.instance.set(5);
await XueHuaAppBadge.instance.set(0);
await XueHuaAppBadge.instance.set(2, windowHandle: hwnd);
```

**Incorrect:**

```dart
await XueHuaAppBadge.instance.set(-1);
final ok = await XueHuaAppBadge.instance.set(5); // set returns void, not bool
await XueHuaAppBadge.instance.setBadge(5); // no such method
```

## `remove`

**Signature:** `Future<void> remove({int? windowHandle})`

**Parameters:**

- `windowHandle` (`int?`, optional, named): forwarded to `set`.

**Implementation:** `await set(0, windowHandle: windowHandle)`.

**Returns:** `Future<void>`.

**Channel:** `'setBadge'`, arguments `{ 'count': 0 }` and optional `'windowHandle'`. Dart does **not** call `'removeBadge'`. Native plugins still implement `'removeBadge'` for other callers; app Dart code must use `remove()`.

**Throws:** same as `set(0)` (`PlatformException` possible; `ArgumentError` is not possible because `count` is `0`).

**Correct:**

```dart
await XueHuaAppBadge.instance.remove();
await XueHuaAppBadge.instance.remove(windowHandle: hwnd);
```

**Incorrect:**

```dart
await XueHuaAppBadge.instance.removeBadge();
await XueHuaAppBadge.instance.remove(5); // no positional count
```

## `requestPermission`

**Signature:** `Future<bool> requestPermission()`

**Parameters:** none.

**Returns:** `true` if the platform granted badge-related permission, or if the channel returns `null` (`result ?? true`). macOS, Windows, and Linux always return `true`. Android API < 33 returns `true` without a dialog.

**Channel:** `'requestPermission'`, arguments `null`.

**Throws:** `PlatformException` on iOS `PERMISSION_ERROR`, Android `NO_ACTIVITY` / `ALREADY_REQUESTING`, or Android `NO_CONTEXT` if the plugin has no application context.

**Behavior:** This is the only API that shows a system permission prompt (iOS notification authorization; Android 13+ `POST_NOTIFICATIONS`). `set()` does not prompt.

**Correct:**

```dart
if (!await XueHuaAppBadge.instance.isPermissionGranted()) {
  final granted = await XueHuaAppBadge.instance.requestPermission();
}
```

**Incorrect:**

```dart
XueHuaAppBadge.instance.requestPermission(); // must await Future<bool>
final granted = XueHuaAppBadge.requestPermission(); // not a static method
```

## `isPermissionGranted`

**Signature:** `Future<bool> isPermissionGranted()`

**Parameters:** none.

**Returns:** `true` if badge permission is already granted, or if the channel returns `null` (`result ?? true`). macOS, Windows, and Linux always return `true`. Android API < 33 always returns `true`.

**Channel:** `'isPermissionGranted'`, arguments `null`.

**Throws:** `PlatformException` if the invocation fails (Android `NO_CONTEXT` if context is null). Does not show a dialog.

**iOS (10+):** `true` only when `authorizationStatus` is `.authorized` or `.provisional` **and** `badgeSetting == .enabled`.

**Correct:**

```dart
final granted = await XueHuaAppBadge.instance.isPermissionGranted();
```

**Incorrect:**

```dart
if (XueHuaAppBadge.instance.isPermissionGranted) {} // not a getter
```

## Methods that do not exist

Do not generate any of the following:

- `XueHuaAppBadge.initialize()` — removed in 2.0.0
- `greet()` — never part of the public API
- `XueHuaAppBadge.set(...)` / other static badge methods — instance methods only
- Dart `removeBadge()` — use `remove()`
- A public `MethodChannel` getter or `XueHuaAppBadgePlugin` Dart class
- A `plugin_platform_interface` `PlatformInterface` subclass in `lib/`
