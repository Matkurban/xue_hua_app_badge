#ifndef FLUTTER_PLUGIN_XUE_HUA_APP_BADGE_PLUGIN_H_
#define FLUTTER_PLUGIN_XUE_HUA_APP_BADGE_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

namespace xue_hua_app_badge {

class XueHuaAppBadgePlugin : public flutter::Plugin {
 public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows *registrar);

  XueHuaAppBadgePlugin(flutter::PluginRegistrarWindows *registrar);

  virtual ~XueHuaAppBadgePlugin();

  XueHuaAppBadgePlugin(const XueHuaAppBadgePlugin&) = delete;
  XueHuaAppBadgePlugin& operator=(const XueHuaAppBadgePlugin&) = delete;

 private:
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);

  flutter::PluginRegistrarWindows *registrar_;
};

}  // namespace xue_hua_app_badge

#endif  // FLUTTER_PLUGIN_XUE_HUA_APP_BADGE_PLUGIN_H_
