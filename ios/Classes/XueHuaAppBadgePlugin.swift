import Flutter
import UIKit
import UserNotifications

public class XueHuaAppBadgePlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    let channel = FlutterMethodChannel(
      name: "xue_hua_app_badge",
      binaryMessenger: registrar.messenger()
    )
    let instance = XueHuaAppBadgePlugin()
    registrar.addMethodCallDelegate(instance, channel: channel)
  }

  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
    switch call.method {
    case "setBadge":
      let args = call.arguments as? [String: Any]
      let count = args?["count"] as? Int ?? 0
      setBadge(count: count, result: result)
    case "removeBadge":
      setBadge(count: 0, result: result)
    case "requestPermission":
      requestPermission(result: result)
    case "isPermissionGranted":
      isPermissionGranted(result: result)
    default:
      result(FlutterMethodNotImplemented)
    }
  }

  private func badgeNumber(for count: Int) -> Int {
    if count <= 0 {
      return 0
    }
    return min(count, 99)
  }

  private func setBadge(count: Int, result: @escaping FlutterResult) {
    let badgeNumber = badgeNumber(for: count)
    DispatchQueue.main.async {
      if #available(iOS 16.0, *) {
        UNUserNotificationCenter.current().setBadgeCount(badgeNumber) { error in
          if let error = error {
            result(
              FlutterError(
                code: "BADGE_ERROR",
                message: error.localizedDescription,
                details: nil
              )
            )
          } else {
            result(nil)
          }
        }
      } else {
        UIApplication.shared.applicationIconBadgeNumber = badgeNumber
        result(nil)
      }
    }
  }

  private func requestPermission(result: @escaping FlutterResult) {
    if #available(iOS 16.0, *) {
      UNUserNotificationCenter.current().requestAuthorization(options: [.badge]) {
        granted,
        error in
        if let error = error {
          result(
            FlutterError(
              code: "PERMISSION_ERROR",
              message: error.localizedDescription,
              details: nil
            )
          )
        } else {
          result(granted)
        }
      }
    } else {
      result(true)
    }
  }

  private func isPermissionGranted(result: @escaping FlutterResult) {
    if #available(iOS 16.0, *) {
      UNUserNotificationCenter.current().getNotificationSettings { settings in
        switch settings.authorizationStatus {
        case .authorized, .provisional:
          result(true)
        default:
          result(false)
        }
      }
    } else {
      result(true)
    }
  }
}
