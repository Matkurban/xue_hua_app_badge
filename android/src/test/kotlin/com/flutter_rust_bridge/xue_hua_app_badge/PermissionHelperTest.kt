package com.kurban.xue_hua_app_badge

import android.Manifest
import android.app.Activity
import android.content.pm.PackageManager
import android.os.Build
import io.flutter.plugin.common.MethodChannel
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.Robolectric
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config

@RunWith(RobolectricTestRunner::class)
class PermissionHelperTest {
    private class TestResult : MethodChannel.Result {
        var successResult: Any? = null
        var errorCode: String? = null
        var errorMessage: String? = null
        var errorDetails: Any? = null
        var notImplementedCalled = false

        override fun success(result: Any?) {
            successResult = result
        }

        override fun error(errorCode: String, errorMessage: String?, errorDetails: Any?) {
            this.errorCode = errorCode
            this.errorMessage = errorMessage
            this.errorDetails = errorDetails
        }

        override fun notImplemented() {
            notImplementedCalled = true
        }
    }

    @Test
    @Config(sdk = [Build.VERSION_CODES.S_V2])
    fun preApi33AlwaysGranted() {
        val activity = Robolectric.buildActivity(Activity::class.java).setup().get()
        assertTrue(PermissionHelper.isBadgePermissionGranted(activity))

        val helper = PermissionHelper()
        val result = TestResult()
        helper.requestBadgePermission(activity, result)
        assertEquals(true, result.successResult)
    }

    @Test
    @Config(sdk = [Build.VERSION_CODES.TIRAMISU])
    fun api33DeniedUntilGranted() {
        val activity = Robolectric.buildActivity(Activity::class.java).setup().get()
        assertFalse(PermissionHelper.isBadgePermissionGranted(activity))
    }

    @Test
    @Config(sdk = [Build.VERSION_CODES.TIRAMISU])
    fun testRequestAndResultCallback() {
        val activity = Robolectric.buildActivity(Activity::class.java).setup().get()
        val helper = PermissionHelper()
        val result = TestResult()

        helper.requestBadgePermission(activity, result)
        helper.onRequestPermissionsResult(
            REQUEST_CODE,
            arrayOf(Manifest.permission.POST_NOTIFICATIONS),
            intArrayOf(PackageManager.PERMISSION_GRANTED),
        )
        assertEquals(true, result.successResult)
    }

    private companion object {
        const val REQUEST_CODE = 0x5876
    }
}
