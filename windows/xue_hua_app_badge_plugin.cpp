#include "xue_hua_app_badge_plugin.h"

#include <windows.h>
#include <shobjidl.h>

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>
#include <flutter/standard_method_codec.h>

#include <memory>
#include <string>

namespace xue_hua_app_badge {

// static
void XueHuaAppBadgePlugin::RegisterWithRegistrar(
    flutter::PluginRegistrarWindows *registrar) {
  auto channel =
      std::make_unique<flutter::MethodChannel<flutter::EncodableValue>>(
          registrar->messenger(), "xue_hua_app_badge",
          &flutter::StandardMethodCodec::GetInstance());

  auto plugin = std::make_unique<XueHuaAppBadgePlugin>(registrar);

  channel->SetMethodCallHandler(
      [plugin_pointer = plugin.get()](const auto &call, auto result) {
        plugin_pointer->HandleMethodCall(call, std::move(result));
      });

  registrar->AddPlugin(std::move(plugin));
}

XueHuaAppBadgePlugin::XueHuaAppBadgePlugin(flutter::PluginRegistrarWindows *registrar)
    : registrar_(registrar) {}

XueHuaAppBadgePlugin::~XueHuaAppBadgePlugin() {}

static HICON CreateBadgeIcon(int count) {
  if (count <= 0) return NULL;
  
  int size = 16;
  
  HDC hdcScreen = GetDC(NULL);
  HDC hdcMem = CreateCompatibleDC(hdcScreen);
  HBITMAP hbmp = CreateCompatibleBitmap(hdcScreen, size, size);
  HBITMAP hbmpOld = (HBITMAP)SelectObject(hdcMem, hbmp);
  
  RECT rect = {0, 0, size, size};
  HBRUSH hBrushBg = CreateSolidBrush(RGB(255, 0, 0));
  SelectObject(hdcMem, GetStockObject(NULL_PEN));
  SelectObject(hdcMem, hBrushBg);
  Ellipse(hdcMem, 0, 0, size, size);
  DeleteObject(hBrushBg);
  
  SetBkMode(hdcMem, TRANSPARENT);
  SetTextColor(hdcMem, RGB(255, 255, 255));
  HFONT hFont = CreateFontA(11, 0, 0, 0, FW_BOLD, FALSE, FALSE, FALSE,
                            DEFAULT_CHARSET, OUT_DEFAULT_PRECIS,
                            CLIP_DEFAULT_PRECIS, DEFAULT_QUALITY,
                            DEFAULT_PITCH | FF_SWISS, "Arial");
  HFONT hFontOld = (HFONT)SelectObject(hdcMem, hFont);
  
  std::string label = count > 99 ? "99" : std::to_string(count);
  DrawTextA(hdcMem, label.c_str(), -1, &rect, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
  
  SelectObject(hdcMem, hFontOld);
  DeleteObject(hFont);
  SelectObject(hdcMem, hbmpOld);
  DeleteDC(hdcMem);
  ReleaseDC(NULL, hdcScreen);
  
  HBITMAP hbmpMask = CreateBitmap(size, size, 1, 1, NULL);
  ICONINFO iconInfo = {0};
  iconInfo.fIcon = TRUE;
  iconInfo.hbmMask = hbmpMask;
  iconInfo.hbmColor = hbmp;
  
  HICON hIcon = CreateIconIndirect(&iconInfo);
  DeleteObject(hbmp);
  DeleteObject(hbmpMask);
  return hIcon;
}

void XueHuaAppBadgePlugin::HandleMethodCall(
    const flutter::MethodCall<flutter::EncodableValue> &method_call,
    std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result) {
  if (method_call.method_name().compare("isSupported") == 0) {
    result->Success(flutter::EncodableValue(true));
  } else if (method_call.method_name().compare("setBadge") == 0) {
    int count = 0;
    const auto *arguments = std::get_if<flutter::EncodableMap>(method_call.arguments());
    if (arguments) {
      auto count_it = arguments->find(flutter::EncodableValue("count"));
      if (count_it != arguments->end()) {
        if (std::holds_alternative<int>(count_it->second)) {
          count = std::get<int>(count_it->second);
        }
      }
    }
    
    HWND hwnd = registrar_->GetView()->GetNativeWindow();
    if (hwnd) {
      ITaskbarList3* pTaskbar = NULL;
      HRESULT hr = CoCreateInstance(CLSID_TaskbarList, NULL, CLSCTX_INPROC_SERVER,
                                    IID_PPV_ARGS(&pTaskbar));
      if (SUCCEEDED(hr) && pTaskbar) {
        pTaskbar->HrInit();
        if (count <= 0) {
          pTaskbar->SetOverlayIcon(hwnd, NULL, L"");
        } else {
          HICON hIcon = CreateBadgeIcon(count);
          pTaskbar->SetOverlayIcon(hwnd, hIcon, L"Badge");
          if (hIcon) DestroyIcon(hIcon);
        }
        pTaskbar->Release();
      }
    }
    result->Success(flutter::EncodableValue(true));
  } else if (method_call.method_name().compare("removeBadge") == 0) {
    HWND hwnd = registrar_->GetView()->GetNativeWindow();
    if (hwnd) {
      ITaskbarList3* pTaskbar = NULL;
      HRESULT hr = CoCreateInstance(CLSID_TaskbarList, NULL, CLSCTX_INPROC_SERVER,
                                    IID_PPV_ARGS(&pTaskbar));
      if (SUCCEEDED(hr) && pTaskbar) {
        pTaskbar->HrInit();
        pTaskbar->SetOverlayIcon(hwnd, NULL, L"");
        pTaskbar->Release();
      }
    }
    result->Success(flutter::EncodableValue(true));
  } else if (method_call.method_name().compare("requestPermission") == 0 ||
             method_call.method_name().compare("isPermissionGranted") == 0) {
    result->Success(flutter::EncodableValue(true));
  } else {
    result->NotImplemented();
  }
}

}  // namespace xue_hua_app_badge
