package com.flutter_rust_bridge.xue_hua_app_badge

import android.Manifest
import android.app.Activity
import android.content.Context
import android.content.pm.PackageManager
import android.os.Build
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat

object PermissionHelper {
    private const val REQUEST_CODE = 0x5876

    @Volatile
    private var pendingCallback: ((Boolean) -> Unit)? = null

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

    @JvmStatic
    fun hasPendingPermissionRequest(): Boolean = pendingCallback != null

    @JvmStatic
    fun requestBadgePermission(activity: Activity, callback: (Boolean) -> Unit) {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
            callback(true)
            return
        }
        if (isBadgePermissionGranted(activity)) {
            callback(true)
            return
        }

        pendingCallback = callback
        activity.runOnUiThread {
            ActivityCompat.requestPermissions(
                activity,
                arrayOf(Manifest.permission.POST_NOTIFICATIONS),
                REQUEST_CODE,
            )
        }
    }

    fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray,
    ): Boolean {
        if (requestCode != REQUEST_CODE) {
            return false
        }
        val granted = grantResults.isNotEmpty() &&
            grantResults[0] == PackageManager.PERMISSION_GRANTED
        val callback = pendingCallback
        pendingCallback = null
        callback?.invoke(granted)
        return true
    }
}
