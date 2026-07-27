package com.kurban.xue_hua_app_badge

import android.content.Context
import androidx.annotation.Keep
import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.embedding.engine.plugins.activity.ActivityAware
import io.flutter.embedding.engine.plugins.activity.ActivityPluginBinding
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugin.common.MethodChannel.MethodCallHandler
import io.flutter.plugin.common.MethodChannel.Result

@Keep
class XueHuaAppBadgePlugin : FlutterPlugin, ActivityAware, MethodCallHandler {
    private var channel: MethodChannel? = null
    private var context: Context? = null
    private var activityBinding: ActivityPluginBinding? = null
    private val permissionHelper = PermissionHelper()

    override fun onAttachedToEngine(binding: FlutterPlugin.FlutterPluginBinding) {
        context = binding.applicationContext
        channel = MethodChannel(binding.binaryMessenger, "xue_hua_app_badge")
        channel?.setMethodCallHandler(this)
    }

    override fun onDetachedFromEngine(binding: FlutterPlugin.FlutterPluginBinding) {
        channel?.setMethodCallHandler(null)
        channel = null
        context = null
    }

    override fun onAttachedToActivity(binding: ActivityPluginBinding) {
        activityBinding = binding
        binding.addRequestPermissionsResultListener(permissionHelper)
    }

    override fun onDetachedFromActivityForConfigChanges() {
        activityBinding?.removeRequestPermissionsResultListener(permissionHelper)
        activityBinding = null
    }

    override fun onReattachedToActivityForConfigChanges(binding: ActivityPluginBinding) {
        activityBinding = binding
        binding.addRequestPermissionsResultListener(permissionHelper)
    }

    override fun onDetachedFromActivity() {
        activityBinding?.removeRequestPermissionsResultListener(permissionHelper)
        activityBinding = null
    }

    override fun onMethodCall(call: MethodCall, result: Result) {
        val ctx = context ?: run {
            result.error("NO_CONTEXT", "Application context is null", null)
            return
        }

        when (call.method) {
            "isSupported" -> {
                result.success(true)
            }
            "setBadge" -> {
                val count = call.argument<Int>("count") ?: 0
                val applied = BadgeHelper.applyBadge(ctx, count)
                result.success(applied)
            }
            "removeBadge" -> {
                val applied = BadgeHelper.applyBadge(ctx, 0)
                result.success(applied)
            }
            "requestPermission" -> {
                permissionHelper.requestBadgePermission(activityBinding?.activity, result)
            }
            "isPermissionGranted" -> {
                val granted = PermissionHelper.isBadgePermissionGranted(ctx)
                result.success(granted)
            }
            else -> {
                result.notImplemented()
            }
        }
    }
}
