#if os(macOS)
    import AppKit
    import FlutterMacOS

    public class XueHuaAppBadgePlugin: NSObject, FlutterPlugin {
        public static func register(with registrar: FlutterPluginRegistrar) {
            let channel = FlutterMethodChannel(name: "xue_hua_app_badge", binaryMessenger: registrar.messenger)
            let instance = XueHuaAppBadgePlugin()
            registrar.addMethodCallDelegate(instance, channel: channel)
        }

        public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
            switch call.method {
            case "isSupported":
                result(true)
            case "setBadge":
                guard let args = call.arguments as? [String: Any],
                      let count = args["count"] as? Int
                else {
                    result(FlutterError(code: "INVALID_ARGUMENT", message: "Count argument missing", details: nil))
                    return
                }
                setBadgeCount(count, result: result)
            case "removeBadge":
                setBadgeCount(0, result: result)
            case "requestPermission":
                result(true)
            case "isPermissionGranted":
                result(true)
            default:
                result(FlutterMethodNotImplemented)
            }
        }

        private func setBadgeCount(_ count: Int, result: @escaping FlutterResult) {
            DispatchQueue.main.async {
                if count <= 0 {
                    NSApp.dockTile.badgeLabel = nil
                } else {
                    let label = count > 99 ? "99+" : "\(count)"
                    NSApp.dockTile.badgeLabel = label
                }
                NSApp.dockTile.display()
                result(true)
            }
        }
    }
#endif
