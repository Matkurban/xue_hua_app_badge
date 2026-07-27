#include "xue_hua_app_badge_plugin.h"

#include <windows.h>
#include <shobjidl.h>
#include <gdiplus.h>

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>
#include <flutter/standard_method_codec.h>

#include <algorithm>
#include <memory>
#include <string>

#pragma comment(lib, "gdiplus.lib")

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
    : registrar_(registrar) {
  Gdiplus::GdiplusStartupInput gdiplusStartupInput;
  Gdiplus::GdiplusStartup(&gdiplus_token_, &gdiplusStartupInput, NULL);
}

XueHuaAppBadgePlugin::~XueHuaAppBadgePlugin() {
  if (gdiplus_token_) {
    Gdiplus::GdiplusShutdown(gdiplus_token_);
    gdiplus_token_ = 0;
  }
}

static HICON CreateBadgeIcon(int count) {
  if (count <= 0) return NULL;

  int systemSmallIcon = GetSystemMetrics(SM_CXSMICON);
  int size = (systemSmallIcon > 16) ? systemSmallIcon * 2 : 32;

  Gdiplus::Bitmap bitmap(size, size, PixelFormat32bppARGB);
  Gdiplus::Graphics graphics(&bitmap);

  graphics.SetSmoothingMode(Gdiplus::SmoothingModeAntiAlias);
  graphics.SetTextRenderingHint(Gdiplus::TextRenderingHintAntiAliasGridFit);
  graphics.SetInterpolationMode(Gdiplus::InterpolationModeHighQualityBicubic);

  graphics.Clear(Gdiplus::Color(0, 0, 0, 0));

  Gdiplus::REAL margin = 1.0f;
  Gdiplus::REAL circleSize = static_cast<Gdiplus::REAL>(size) - 2 * margin;
  Gdiplus::SolidBrush redBrush(Gdiplus::Color(255, 235, 32, 32));
  graphics.FillEllipse(&redBrush, margin, margin, circleSize, circleSize);

  std::string label = count > 99 ? "99+" : std::to_string(count);
  std::wstring wlabel(label.begin(), label.end());

  Gdiplus::REAL fontSize = circleSize * 0.55f;
  if (label.length() == 2) {
    fontSize = circleSize * 0.46f;
  } else if (label.length() > 2) {
    fontSize = circleSize * 0.38f;
  }

  Gdiplus::FontFamily fontFamily(L"Segoe UI");
  Gdiplus::Font font(&fontFamily, fontSize, Gdiplus::FontStyleBold, Gdiplus::UnitPixel);
  Gdiplus::SolidBrush whiteBrush(Gdiplus::Color(255, 255, 255, 255));

  Gdiplus::StringFormat format;
  format.SetAlignment(Gdiplus::StringAlignmentCenter);
  format.SetLineAlignment(Gdiplus::StringAlignmentCenter);

  Gdiplus::RectF layoutRect(0, 0, static_cast<Gdiplus::REAL>(size), static_cast<Gdiplus::REAL>(size));
  graphics.DrawString(wlabel.c_str(), -1, &font, layoutRect, &format, &whiteBrush);

  HICON hIcon = NULL;
  bitmap.GetHICON(&hIcon);

  return hIcon;
}

static HWND GetTargetWindow(flutter::PluginRegistrarWindows* registrar, const flutter::EncodableMap* arguments) {
  HWND hwnd = nullptr;
  if (arguments) {
    auto handle_it = arguments->find(flutter::EncodableValue("windowHandle"));
    if (handle_it != arguments->end()) {
      if (std::holds_alternative<int64_t>(handle_it->second)) {
        hwnd = reinterpret_cast<HWND>(std::get<int64_t>(handle_it->second));
      } else if (std::holds_alternative<int>(handle_it->second)) {
        hwnd = reinterpret_cast<HWND>(static_cast<intptr_t>(std::get<int>(handle_it->second)));
      }
    }
  }
  if (!hwnd && registrar && registrar->GetView()) {
    hwnd = registrar->GetView()->GetNativeWindow();
  }
  if (hwnd) {
    HWND root_hwnd = GetAncestor(hwnd, GA_ROOT);
    if (root_hwnd) {
      return root_hwnd;
    }
  }
  return hwnd;
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
        } else if (std::holds_alternative<int64_t>(count_it->second)) {
          count = static_cast<int>(std::get<int64_t>(count_it->second));
        }
      }
    }

    HWND hwnd = GetTargetWindow(registrar_, arguments);
    if (hwnd) {
      HRESULT hrCom = CoInitializeEx(NULL, COINIT_APARTMENTTHREADED);
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
      if (SUCCEEDED(hrCom)) {
        CoUninitialize();
      }
    }
    result->Success(flutter::EncodableValue(true));
  } else if (method_call.method_name().compare("removeBadge") == 0) {
    const auto *arguments = std::get_if<flutter::EncodableMap>(method_call.arguments());
    HWND hwnd = GetTargetWindow(registrar_, arguments);
    if (hwnd) {
      HRESULT hrCom = CoInitializeEx(NULL, COINIT_APARTMENTTHREADED);
      ITaskbarList3* pTaskbar = NULL;
      HRESULT hr = CoCreateInstance(CLSID_TaskbarList, NULL, CLSCTX_INPROC_SERVER,
                                    IID_PPV_ARGS(&pTaskbar));
      if (SUCCEEDED(hr) && pTaskbar) {
        pTaskbar->HrInit();
        pTaskbar->SetOverlayIcon(hwnd, NULL, L"");
        pTaskbar->Release();
      }
      if (SUCCEEDED(hrCom)) {
        CoUninitialize();
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
