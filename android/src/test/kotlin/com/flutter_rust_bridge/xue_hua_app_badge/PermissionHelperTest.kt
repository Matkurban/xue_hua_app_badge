package com.flutter_rust_bridge.xue_hua_app_badge

import android.Manifest
import android.app.Activity
import android.content.pm.PackageManager
import android.os.Build
import android.os.Looper
import org.junit.Assert.assertFalse
import org.junit.Assert.assertSame
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.Robolectric
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config
import kotlin.system.measureTimeMillis

@RunWith(RobolectricTestRunner::class)
class PermissionHelperTest {
    @Test
    @Config(sdk = [Build.VERSION_CODES.S_V2])
    fun preApi33AlwaysGranted() {
        val activity = Robolectric.buildActivity(Activity::class.java).setup().get()
        assertTrue(PermissionHelper.isBadgePermissionGranted(activity))
        assertTrue(PermissionHelper.requestBadgePermission(activity))
    }

    @Test
    @Config(sdk = [Build.VERSION_CODES.TIRAMISU])
    fun api33DeniedUntilGranted() {
        val activity = Robolectric.buildActivity(Activity::class.java).setup().get()
        assertFalse(PermissionHelper.isBadgePermissionGranted(activity))
    }

    @Test
    @Config(sdk = [Build.VERSION_CODES.TIRAMISU])
    fun mainThreadRequestReturnsWithoutWaiting() {
        val activity = Robolectric.buildActivity(Activity::class.java).setup().get()
        assertSame(Looper.getMainLooper(), Looper.myLooper())

        // The permission result is delivered on this very thread, so waiting for it here
        // used to hang for the full 30s timeout and trigger an ANR.
        val elapsed = measureTimeMillis {
            assertFalse(PermissionHelper.requestBadgePermission(activity))
        }
        assertTrue("blocked the main thread for ${elapsed}ms", elapsed < 1_000)

        // Release the in-flight request so it cannot leak into other tests.
        PermissionHelper.onRequestPermissionsResult(
            REQUEST_CODE,
            arrayOf(Manifest.permission.POST_NOTIFICATIONS),
            intArrayOf(PackageManager.PERMISSION_DENIED),
        )
    }

    private companion object {
        /** Mirrors the private request code in [PermissionHelper]. */
        const val REQUEST_CODE = 0x5876
    }
}
