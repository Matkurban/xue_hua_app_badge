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
import java.util.concurrent.atomic.AtomicLong

object PermissionHelper {
    private const val REQUEST_CODE = 0x5876
    private const val TIMEOUT_SECONDS = 30L

    private val nextRequestId = AtomicLong(0)

    private val lock = Any()

    @Volatile
    private var inFlight: InFlightRequest? = null

    private data class InFlightRequest(
        val id: Long,
        val latch: CountDownLatch,
        var result: Boolean = false,
    )

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

        val (request, isOwner) = synchronized(lock) {
            val active = inFlight
            if (active != null) {
                Pair(active, false)
            } else {
                val newRequest = InFlightRequest(
                    id = nextRequestId.incrementAndGet(),
                    latch = CountDownLatch(1),
                )
                inFlight = newRequest
                Pair(newRequest, true)
            }
        }

        if (!isOwner) {
            request.latch.await(TIMEOUT_SECONDS, TimeUnit.SECONDS)
            return isBadgePermissionGranted(activity)
        }

        ActivityCompat.requestPermissions(
            activity,
            arrayOf(Manifest.permission.POST_NOTIFICATIONS),
            REQUEST_CODE,
        )

        request.latch.await(TIMEOUT_SECONDS, TimeUnit.SECONDS)

        synchronized(lock) {
            if (inFlight?.id == request.id) {
                inFlight = null
            }
        }

        return request.result
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

        synchronized(lock) {
            inFlight?.let { request ->
                request.result = granted
                request.latch.countDown()
                inFlight = null
            }
        }
        return true
    }
}
