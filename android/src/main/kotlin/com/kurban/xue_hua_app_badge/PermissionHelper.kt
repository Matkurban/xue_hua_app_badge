package com.kurban.xue_hua_app_badge

import android.Manifest
import android.app.Activity
import android.content.Context
import android.content.pm.PackageManager
import android.os.Build
import androidx.annotation.Keep
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugin.common.PluginRegistry

@Keep
class PermissionHelper : PluginRegistry.RequestPermissionsResultListener {
    companion object {
        private const val REQUEST_CODE = 0x5876

        @JvmStatic
        fun isBadgePermissionGranted(context: Context): Boolean {
            if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
                return true
            }
            return ContextCompat.checkSelfPermission(
                context,
                Manifest.permission.POST_NOTIFICATIONS,
            ) == PackageManager.PERMISSION_GRANTED
        }
    }

    private var pendingResult: MethodChannel.Result? = null

    fun requestBadgePermission(activity: Activity?, result: MethodChannel.Result) {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
            result.success(true)
            return
        }
        if (activity == null) {
            result.error("NO_ACTIVITY", "Permission request requires an active Activity", null)
            return
        }
        if (isBadgePermissionGranted(activity)) {
            result.success(true)
            return
        }

        if (pendingResult != null) {
            result.error("ALREADY_REQUESTING", "A permission request is already in progress", null)
            return
        }

        pendingResult = result
        ActivityCompat.requestPermissions(
            activity,
            arrayOf(Manifest.permission.POST_NOTIFICATIONS),
            REQUEST_CODE,
        )
    }

    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray,
    ): Boolean {
        if (requestCode != REQUEST_CODE) {
            return false
        }

        val result = pendingResult ?: return false
        pendingResult = null

        val granted = grantResults.isNotEmpty() &&
                grantResults[0] == PackageManager.PERMISSION_GRANTED
        result.success(granted)
        return true
    }
}
