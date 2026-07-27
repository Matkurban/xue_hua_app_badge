#if os(iOS)
import Flutter
import UIKit
import UserNotifications

public class XueHuaAppBadgePlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    let channel = FlutterMethodChannel(name: "xue_hua_app_badge", binaryMessenger: registrar.messenger())
    let instance = XueHuaAppBadgePlugin()
    registrar.addMethodCallDelegate(instance, channel: channel)
  }

  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
    switch call.method {
    case "isSupported":
      result(true)
    case "setBadge":
      guard let args = call.arguments as? [String: Any],
            let count = args["count"] as? Int else {
        result(FlutterError(code: "INVALID_ARGUMENT", message: "Count argument missing", details: nil))
        return
      }
      setBadgeCount(count, result: result)
    case "removeBadge":
      setBadgeCount(0, result: result)
    case "requestPermission":
      requestPermission(result: result)
    case "isPermissionGranted":
      checkPermission(result: result)
    default:
      result(FlutterMethodNotImplemented)
    }
  }

  private func setBadgeCount(_ count: Int, result: @escaping FlutterResult) {
    let safeCount = max(0, count)
    if #available(iOS 16.0, *) {
      UNUserNotificationCenter.current().setBadgeCount(safeCount) { error in
        DispatchQueue.main.async {
          if let error = error {
            result(FlutterError(code: "SET_BADGE_FAILED", message: error.localizedDescription, details: nil))
          } else {
            result(true)
          }
        }
      }
    } else {
      DispatchQueue.main.async {
        UIApplication.shared.applicationIconBadgeNumber = safeCount
        result(true)
      }
    }
  }

  private func requestPermission(result: @escaping FlutterResult) {
    if #available(iOS 10.0, *) {
      UNUserNotificationCenter.current().requestAuthorization(options: [.badge, .alert, .sound]) { granted, error in
        DispatchQueue.main.async {
          if let error = error {
            result(FlutterError(code: "PERMISSION_ERROR", message: error.localizedDescription, details: nil))
          } else {
            result(granted)
          }
        }
      }
    } else {
      result(true)
    }
  }

  private func checkPermission(result: @escaping FlutterResult) {
    if #available(iOS 10.0, *) {
      UNUserNotificationCenter.current().getNotificationSettings { settings in
        DispatchQueue.main.async {
          let granted = settings.authorizationStatus == .authorized || settings.authorizationStatus == .provisional
          let badgeEnabled = settings.badgeSetting == .enabled
          result(granted && badgeEnabled)
        }
      }
    } else {
      result(true)
    }
  }
}
#endif
