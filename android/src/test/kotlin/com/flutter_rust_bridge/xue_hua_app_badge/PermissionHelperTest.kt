package com.flutter_rust_bridge.xue_hua_app_badge

import android.app.Activity
import android.os.Build
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.Robolectric
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config

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
}
