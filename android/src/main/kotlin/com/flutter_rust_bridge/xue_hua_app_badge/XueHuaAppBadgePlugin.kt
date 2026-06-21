package com.flutter_rust_bridge.xue_hua_app_badge

import android.app.Activity
import android.content.Context
import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.embedding.engine.plugins.activity.ActivityAware
import io.flutter.embedding.engine.plugins.activity.ActivityPluginBinding
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugin.common.PluginRegistry

class XueHuaAppBadgePlugin :
    FlutterPlugin,
    MethodChannel.MethodCallHandler,
    ActivityAware,
    PluginRegistry.RequestPermissionsResultListener {
    private lateinit var applicationContext: Context
    private lateinit var channel: MethodChannel
    private var activity: Activity? = null
    private var activityBinding: ActivityPluginBinding? = null

    override fun onAttachedToEngine(binding: FlutterPlugin.FlutterPluginBinding) {
        applicationContext = binding.applicationContext
        channel = MethodChannel(binding.binaryMessenger, "xue_hua_app_badge")
        channel.setMethodCallHandler(this)
    }

    override fun onDetachedFromEngine(binding: FlutterPlugin.FlutterPluginBinding) {
        channel.setMethodCallHandler(null)
    }

    override fun onMethodCall(call: MethodCall, result: MethodChannel.Result) {
        when (call.method) {
            "setBadge" -> {
                val count = call.argument<Int>("count") ?: 0
                applyBadge(count, result)
            }
            "removeBadge" -> applyBadge(0, result)
            "isPermissionGranted" -> {
                result.success(
                    PermissionHelper.isBadgePermissionGranted(applicationContext),
                )
            }
            "requestPermission" -> {
                val currentActivity = activity
                if (currentActivity == null) {
                    result.error(
                        "NO_ACTIVITY",
                        "Android activity not available; ensure Flutter activity is attached",
                        null,
                    )
                    return
                }
                if (PermissionHelper.hasPendingPermissionRequest()) {
                    result.error(
                        "PERMISSION_IN_PROGRESS",
                        "A badge permission request is already in progress",
                        null,
                    )
                    return
                }
                PermissionHelper.requestBadgePermission(currentActivity) { granted ->
                    result.success(granted)
                }
            }
            else -> result.notImplemented()
        }
    }

    private fun applyBadge(count: Int, result: MethodChannel.Result) {
        val applied = BadgeHelper.applyBadge(applicationContext, count)
        if (applied) {
            result.success(null)
        } else {
            result.error(
                "BADGE_ERROR",
                "BadgeHelper.applyBadge returned false",
                null,
            )
        }
    }

    override fun onAttachedToActivity(binding: ActivityPluginBinding) {
        activity = binding.activity
        activityBinding = binding
        binding.addRequestPermissionsResultListener(this)
    }

    override fun onDetachedFromActivityForConfigChanges() {
        activity = null
        activityBinding?.removeRequestPermissionsResultListener(this)
        activityBinding = null
    }

    override fun onReattachedToActivityForConfigChanges(binding: ActivityPluginBinding) {
        activity = binding.activity
        activityBinding = binding
        binding.addRequestPermissionsResultListener(this)
    }

    override fun onDetachedFromActivity() {
        activity = null
        activityBinding?.removeRequestPermissionsResultListener(this)
        activityBinding = null
    }

    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray,
    ): Boolean {
        return PermissionHelper.onRequestPermissionsResult(
            requestCode,
            permissions,
            grantResults,
        )
    }
}
