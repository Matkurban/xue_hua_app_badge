package com.kurban.xue_hua_app_badge

import android.content.Context
import android.os.Build
import androidx.test.core.app.ApplicationProvider
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config

@RunWith(RobolectricTestRunner::class)
class BadgeHelperTest {
    @Test
    @Config(sdk = [Build.VERSION_CODES.O])
    fun testApplyBadgeClearsSuccessfully() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val result = BadgeHelper.applyBadge(context, 0)
        assertTrue(result)
    }

    @Test
    @Config(sdk = [Build.VERSION_CODES.P])
    fun testApplyBadgeWithPositiveCount() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val result = BadgeHelper.applyBadge(context, 5)
        assertTrue(result)
    }
}
