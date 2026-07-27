#include "include/xue_hua_app_badge/xue_hua_app_badge_plugin.h"
#include "xue_hua_app_badge_plugin_private.h"

#include <flutter_linux/flutter_linux.h>
#include <gio/gio.h>
#include <gtk/gtk.h>

#include <cstring>
#include <string>

#define XUE_HUA_APP_BADGE_PLUGIN(obj) \
  (G_TYPE_CHECK_INSTANCE_CAST((obj), xue_hua_app_badge_plugin_get_type(), \
                              XueHuaAppBadgePlugin))

G_DEFINE_TYPE(XueHuaAppBadgePlugin, xue_hua_app_badge_plugin, g_object_get_type())

static void emit_unity_badge_signal(int count) {
  GError* error = NULL;
  GDBusConnection* connection = g_bus_get_sync(G_BUS_TYPE_SESSION, NULL, &error);
  if (!connection) {
    if (error) g_error_free(error);
    return;
  }

  const char* desktop_id = g_getenv("GIO_LAUNCHED_DESKTOP_FILE");
  if (!desktop_id) {
    desktop_id = g_getenv("GAPPLICATION_ID");
  }
  std::string app_uri = "application://";
  if (desktop_id && strlen(desktop_id) > 0) {
    app_uri += desktop_id;
  } else {
    app_uri += "flutter.desktop";
  }

  GVariantBuilder builder;
  g_variant_builder_init(&builder, G_VARIANT_TYPE("a{sv}"));

  if (count > 0) {
    g_variant_builder_add(&builder, "{sv}", "count", g_variant_new_int64(count > 99 ? 99 : count));
    g_variant_builder_add(&builder, "{sv}", "count-visible", g_variant_new_boolean(TRUE));
  } else {
    g_variant_builder_add(&builder, "{sv}", "count-visible", g_variant_new_boolean(FALSE));
  }

  GVariant* params = g_variant_new("(sa{sv})", app_uri.c_str(), &builder);

  g_dbus_connection_emit_signal(
      connection, NULL, "/com/canonical/unity/launcherentry/1",
      "com.canonical.Unity.LauncherEntry", "Update", params, &error);

  if (error) g_error_free(error);
  g_object_unref(connection);
}

static void xue_hua_app_badge_plugin_handle_method_call(
    XueHuaAppBadgePlugin* self,
    FlMethodCall* method_call) {
  g_autorelease FlMethodResponse* response = nullptr;

  const gchar* method = fl_method_call_get_name(method_call);

  if (strcmp(method, "isSupported") == 0) {
    g_autorelease FlValue* value = fl_value_new_bool(TRUE);
    response = FL_METHOD_RESPONSE(fl_method_success_response_new(value));
  } else if (strcmp(method, "setBadge") == 0) {
    FlValue* args = fl_method_call_get_args(method_call);
    int count = 0;
    if (fl_value_get_type(args) == FL_VALUE_TYPE_MAP) {
      FlValue* count_val = fl_value_lookup_string(args, "count");
      if (count_val && fl_value_get_type(count_val) == FL_VALUE_TYPE_INT) {
        count = fl_value_get_int(count_val);
      }
    }
    emit_unity_badge_signal(count);
    g_autorelease FlValue* value = fl_value_new_bool(TRUE);
    response = FL_METHOD_RESPONSE(fl_method_success_response_new(value));
  } else if (strcmp(method, "removeBadge") == 0) {
    emit_unity_badge_signal(0);
    g_autorelease FlValue* value = fl_value_new_bool(TRUE);
    response = FL_METHOD_RESPONSE(fl_method_success_response_new(value));
  } else if (strcmp(method, "requestPermission") == 0 ||
             strcmp(method, "isPermissionGranted") == 0) {
    g_autorelease FlValue* value = fl_value_new_bool(TRUE);
    response = FL_METHOD_RESPONSE(fl_method_success_response_new(value));
  } else {
    response = FL_METHOD_RESPONSE(fl_method_not_implemented_response_new());
  }

  fl_method_call_respond(method_call, response, nullptr);
}

static void xue_hua_app_badge_plugin_dispose(GObject* object) {
  G_OBJECT_CLASS(xue_hua_app_badge_plugin_parent_class)->dispose(object);
}

static void xue_hua_app_badge_plugin_class_init(XueHuaAppBadgePluginClass* klass) {
  G_OBJECT_CLASS(klass)->dispose = xue_hua_app_badge_plugin_dispose;
}

static void xue_hua_app_badge_plugin_init(XueHuaAppBadgePlugin* self) {}

static void method_call_cb(FlMethodChannel* channel, FlMethodCall* method_call,
                           gpointer user_data) {
  XueHuaAppBadgePlugin* plugin = XUE_HUA_APP_BADGE_PLUGIN(user_data);
  xue_hua_app_badge_plugin_handle_method_call(plugin, method_call);
}

void xue_hua_app_badge_plugin_register_with_registrar(FlPluginRegistrar* registrar) {
  XueHuaAppBadgePlugin* plugin = XUE_HUA_APP_BADGE_PLUGIN(
      g_object_new(xue_hua_app_badge_plugin_get_type(), nullptr));

  g_autorelease FlStandardMethodCodec* codec = fl_standard_method_codec_new();
  g_autorelease FlMethodChannel* channel =
      fl_method_channel_new(fl_plugin_registrar_get_messenger(registrar),
                            "xue_hua_app_badge", FL_METHOD_CODEC(codec));
  fl_method_channel_set_method_call_handler(channel, method_call_cb,
                                             g_object_ref(plugin),
                                             g_object_unref);

  g_object_unref(plugin);
}
