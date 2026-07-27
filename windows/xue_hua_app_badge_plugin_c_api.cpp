#include "include/xue_hua_app_badge/xue_hua_app_badge_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "xue_hua_app_badge_plugin.h"

void XueHuaAppBadgePluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  xue_hua_app_badge::XueHuaAppBadgePlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}
