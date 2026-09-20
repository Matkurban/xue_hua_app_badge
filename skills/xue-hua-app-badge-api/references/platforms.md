# Platform behavior for Dart callers

Facts below come from the native plugin sources. Use them when choosing whether to request permission, how to interpret `set()` completing, or how counts appear on the icon.

Supported plugin platforms in `pubspec.yaml`: Android, iOS, Linux, macOS, Windows. There is no Web implementation.

## Count display

Dart accepts any `count >= 0` and sends that integer on `'setBadge'`. Display is platform-specific. There is **no** Dart-side cap at 99.

| Platform | Count `> 99` | Count `<= 0` |
| --- | --- | --- |
| macOS | Dock label `"99+"` | `badgeLabel = nil` |
| Windows | Overlay icon text `"99+"` | Overlay icon cleared |
| iOS | System uses the integer as given | Badge cleared (`max(0, count)`) |
| Android | OEM / notification APIs receive the integer as given | Count coerced with `coerceAtLeast(0)`, then vendor/notification clear |
| Linux | Unity LauncherEntry `count` is the integer as `int64` | `count-visible` is false |

## `windowHandle`

* Optional named `int?` on `set` and `remove`.
* Only Windows reads `'windowHandle'` from the method arguments (as `int` or `int64`, cast to `HWND`).
* If omitted or not a usable HWND, Windows uses `registrar->GetView()->GetNativeWindow()`, then `GetAncestor(..., GA_ROOT)`.
* iOS, Android, macOS, and Linux ignore extra map keys.

## Permissions

`set()` never requests permission.

| Platform | `isPermissionGranted()` | `requestPermission()` |
| --- | --- | --- |
| iOS 10+ | `authorizationStatus` is `.authorized` or `.provisional`, **and** `badgeSetting == .enabled`. Pre-iOS 10 returns `true`. | `UNUserNotificationCenter.requestAuthorization(options: [.badge, .alert, .sound])`. Shows the system dialog when status is not determined. Pre-iOS 10 returns `true`. |
| Android API < 33 | `true` (no `POST_NOTIFICATIONS`) | `true` without a dialog |
| Android API 33+ | `POST_NOTIFICATIONS` granted | Requests `POST_NOTIFICATIONS` on the current Activity |
| macOS | `true` | `true` |
| Windows | `true` | `true` |
| Linux | `true` | `true` |

Android OEM launcher permissions (`CHANGE_BADGE`, vivo badge, Meizu, ShortcutBadger vendor permissions, and `POST_NOTIFICATIONS`) are declared in the plugin `AndroidManifest.xml` and merge into the host app. The host does not need to redeclare them.

## iOS

* iOS 16+: `UNUserNotificationCenter.setBadgeCount`.
* Older iOS: `UIApplication.shared.applicationIconBadgeNumber` on the main queue.
* `'setBadge'` without a `count` `Int` returns FlutterError `INVALID_ARGUMENT` (`Count argument missing`) → Dart `PlatformException`.
* `setBadgeCount` failure → `SET_BADGE_FAILED`.
* `requestAuthorization` failure → `PERMISSION_ERROR`.

## Android

Apply order in `BadgeHelper.applyBadge`:

1. Match `Build.MANUFACTURER` to Honor, Huawei, Xiaomi/Redmi/POCO, OPPO/OnePlus/realme, vivo/iQOO, or Meizu and call that OEM API.
2. If no OEM match, call ShortcutBadger.
3. Unless the manufacturer is Xiaomi/Redmi/POCO, also try a silent `NotificationChannel` dot fallback on API 26+ (`BadgeNotifications.applyDotFallback`). Xiaomi uses notifications as its official path and does not add that extra fallback.

`set()` completing in Dart does **not** mean a numeric desktop badge appeared.

OEM notes:

* **Honor / Huawei:** numeric badge via launcher ContentProvider. Opening the app or dismissing notifications does **not** clear it. Call `remove()` or `set(0)`.
* **Xiaomi / Redmi / POCO:** badge is tied to a silent notification. Progress/ongoing-style notifications are not used; the plugin posts a non-ongoing silent notification (MIUI 12+ uses `setNumber`, older MIUI uses `extraNotification.setMessageCount`). Returns `false` if notifications are disabled.
* **vivo / iQOO:** user must enable the per-app desktop badge switch in system settings.
* **OPPO / OnePlus / realme:** numeric badges are whitelist-gated; other apps often get only a notification dot. Clearing sends launcher count `-1` (internal `launcherCount`: `0` → `-1`).
* **Meizu:** third-party apps often get a dot; Flyme 10+ may accept the numeric provider.

Android channel errors:

* `NO_CONTEXT` — plugin application context is null (`set`, `remove` path via `setBadge`, `isSupported` is not reached the same way; `onMethodCall` fails all methods including permission checks).
* `NO_ACTIVITY` — `requestPermission` on API 33+ with no attached Activity.
* `ALREADY_REQUESTING` — a `POST_NOTIFICATIONS` request is already in flight.

## macOS

* Sets `NSApp.dockTile.badgeLabel` on the main queue, then `display()`.
* `'setBadge'` without `count` → `INVALID_ARGUMENT`.
* Permission methods return `true`.

## Windows

* `ITaskbarList3::SetOverlayIcon` with a GDI+ overlay. Missing/zero count clears the overlay.
* Permission methods return `true`.
* Handler still returns success (`true`) even if no HWND is found; Dart `set`/`remove` still complete as `void`.

## Linux

* Emits D-Bus signal `com.canonical.Unity.LauncherEntry` `Update` on `/com/canonical/unity/launcherentry/1`.
* Desktop file id comes from `GIO_LAUNCHED_DESKTOP_FILE`, else `GAPPLICATION_ID`, else `application://flutter.desktop`.
* The badge appears only if the desktop environment implements Unity LauncherEntry (Unity, some GNOME/KDE setups). Completing `set()` does not guarantee a visible badge.
* Permission methods return `true`.

## `PlatformException` codes that native code actually emits

Use only these codes when handling errors. Do not invent others.

| Code | Platform | When |
| --- | --- | --- |
| `INVALID_ARGUMENT` | iOS, macOS | `'setBadge'` missing `count` |
| `SET_BADGE_FAILED` | iOS 16+ | `setBadgeCount` callback error |
| `PERMISSION_ERROR` | iOS | `requestAuthorization` error |
| `NO_CONTEXT` | Android | application context is null |
| `NO_ACTIVITY` | Android 13+ | `requestPermission` without an Activity |
| `ALREADY_REQUESTING` | Android 13+ | overlapping `requestPermission` |

Dart also throws `ArgumentError` from `set` when `count < 0`.
