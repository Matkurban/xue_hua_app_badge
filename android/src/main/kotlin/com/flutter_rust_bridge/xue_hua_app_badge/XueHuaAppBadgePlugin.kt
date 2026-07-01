package com.flutter_rust_bridge.xue_hua_app_badge

import android.app.Activity
import android.content.Context
import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.embedding.engine.plugins.activity.ActivityAware
import io.flutter.embedding.engine.plugins.activity.ActivityPluginBinding
import io.flutter.plugin.common.PluginRegistry

class XueHuaAppBadgePlugin :
    FlutterPlugin,
    ActivityAware,
    PluginRegistry.RequestPermissionsResultListener {
    companion object {
        init {
            System.loadLibrary("xue_hua_app_badge")
        }

        @JvmStatic
        external fun initAndroid(context: Context)

        @JvmStatic
        external fun initActivity(activity: Activity)

        @JvmStatic
        external fun clearActivity()
    }

    private var activityBinding: ActivityPluginBinding? = null

    override fun onAttachedToEngine(binding: FlutterPlugin.FlutterPluginBinding) {
        initAndroid(binding.applicationContext)
    }

    override fun onDetachedFromEngine(binding: FlutterPlugin.FlutterPluginBinding) {
        // Application context global ref is kept for the process lifetime.
    }

    override fun onAttachedToActivity(binding: ActivityPluginBinding) {
        initActivity(binding.activity)
        activityBinding = binding
        binding.addRequestPermissionsResultListener(this)
    }

    override fun onDetachedFromActivityForConfigChanges() {
        // Keep the Activity global ref during config changes so in-flight permission
        // dialogs can still complete.
        activityBinding?.removeRequestPermissionsResultListener(this)
        activityBinding = null
    }

    override fun onReattachedToActivityForConfigChanges(binding: ActivityPluginBinding) {
        initActivity(binding.activity)
        activityBinding = binding
        binding.addRequestPermissionsResultListener(this)
    }

    override fun onDetachedFromActivity() {
        clearActivity()
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
