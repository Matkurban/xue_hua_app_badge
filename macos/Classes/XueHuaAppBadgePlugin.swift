import AppKit
import FlutterMacOS

public class XueHuaAppBadgePlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    let channel = FlutterMethodChannel(
      name: "xue_hua_app_badge",
      binaryMessenger: registrar.messenger
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
      result(true)
    case "isPermissionGranted":
      result(true)
    default:
      result(FlutterMethodNotImplemented)
    }
  }

  private func badgeLabel(for count: Int) -> String? {
    if count <= 0 {
      return nil
    }
    if count > 99 {
      return "99+"
    }
    return String(count)
  }

  private func setBadge(count: Int, result: @escaping FlutterResult) {
    DispatchQueue.main.async {
      let label = self.badgeLabel(for: count)
      NSApplication.shared.dockTile?.badgeLabel = label
      NSApplication.shared.dockTile?.display()
      result(nil)
    }
  }
}
