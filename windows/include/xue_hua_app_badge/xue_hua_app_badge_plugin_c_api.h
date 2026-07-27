#ifndef FLUTTER_PLUGIN_XUE_HUA_APP_BADGE_PLUGIN_C_API_H_
#define FLUTTER_PLUGIN_XUE_HUA_APP_BADGE_PLUGIN_C_API_H_

#include <flutter_plugin_registrar.h>

#if defined(FLUTTER_PLUGIN_IMPL)
#define FLUTTER_PLUGIN_EXPORT __declspec(dllexport)
#else
#define FLUTTER_PLUGIN_EXPORT __declspec(dllimport)
#endif

#if defined(__cplusplus)
extern "C" {
#endif

FLUTTER_PLUGIN_EXPORT void XueHuaAppBadgePluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar);

#if defined(__cplusplus)
}
#endif

#endif  // FLUTTER_PLUGIN_XUE_HUA_APP_BADGE_PLUGIN_C_API_H_
