package com.flutter_rust_bridge.xue_hua_app_badge

import android.Manifest
import android.app.Activity
import android.content.Context
import android.content.pm.PackageManager
import android.os.Build
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit

object PermissionHelper {
    private const val REQUEST_CODE = 0x5876

    @Volatile
    private var pendingLatch: CountDownLatch? = null

    @Volatile
    private var pendingResult: Boolean = false

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
    fun requestBadgePermission(activity: Activity): Boolean {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
            return true
        }
        if (isBadgePermissionGranted(activity)) {
            return true
        }

        val latch = CountDownLatch(1)
        pendingLatch = latch
        pendingResult = false

        ActivityCompat.requestPermissions(
            activity,
            arrayOf(Manifest.permission.POST_NOTIFICATIONS),
            REQUEST_CODE,
        )

        latch.await(30, TimeUnit.SECONDS)
        pendingLatch = null
        return pendingResult
    }

    fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray,
    ): Boolean {
        if (requestCode != REQUEST_CODE) {
            return false
        }
        pendingResult = grantResults.isNotEmpty() &&
            grantResults[0] == PackageManager.PERMISSION_GRANTED
        pendingLatch?.countDown()
        return true
    }
}
