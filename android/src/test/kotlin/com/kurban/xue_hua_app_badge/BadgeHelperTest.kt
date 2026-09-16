package com.kurban.xue_hua_app_badge

import android.app.Application
import android.app.NotificationManager
import android.content.ContentProvider
import android.content.Context
import android.content.pm.ProviderInfo
import android.os.Build
import androidx.test.core.app.ApplicationProvider
import com.kurban.xue_hua_app_badge.vendor.ChangeBadgeCall
import com.kurban.xue_hua_app_badge.vendor.HonorBadge
import com.kurban.xue_hua_app_badge.vendor.HuaweiBadge
import com.kurban.xue_hua_app_badge.vendor.MeizuBadge
import com.kurban.xue_hua_app_badge.vendor.OppoBadge
import com.kurban.xue_hua_app_badge.vendor.VendorBadges
import com.kurban.xue_hua_app_badge.vendor.VivoBadge
import com.kurban.xue_hua_app_badge.vendor.XiaomiBadge
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.Shadows
import org.robolectric.annotation.Config
import org.robolectric.shadows.ShadowContentResolver
import org.robolectric.util.ReflectionHelpers

@RunWith(RobolectricTestRunner::class)
class BadgeHelperTest {
    private val originalManufacturer: String = Build.MANUFACTURER

    @After
    fun tearDown() {
        setManufacturer(originalManufacturer)
    }

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

    @Test
    fun vendorMatchingUsesManufacturerAliases() {
        assertTrue(HonorBadge.matches("HONOR"))
        assertTrue(HuaweiBadge.matches("HUAWEI"))
        assertTrue(XiaomiBadge.matches("Xiaomi"))
        assertTrue(XiaomiBadge.matches("Redmi"))
        assertTrue(XiaomiBadge.matches("POCO"))
        assertTrue(OppoBadge.matches("OPPO"))
        assertTrue(OppoBadge.matches("OnePlus"))
        assertTrue(OppoBadge.matches("realme"))
        assertTrue(VivoBadge.matches("vivo"))
        assertTrue(VivoBadge.matches("iQOO"))
        assertTrue(MeizuBadge.matches("Meizu"))
        assertFalse(HonorBadge.matches("HUAWEI"))
        assertEquals(HonorBadge, VendorBadges.find("honor"))
        assertEquals(HuaweiBadge, VendorBadges.find("Huawei"))
        assertNull(VendorBadges.find("samsung"))
    }

    @Test
    @Config(sdk = [Build.VERSION_CODES.P])
    fun honorFallsBackToHuaweiUriWhenNewTypeIsEmpty() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val newProvider = RecordingBadgeProvider().apply { type = null }
        val oldProvider = RecordingBadgeProvider().apply { type = "badge" }
        registerProvider(context, HonorBadge.URI_NEW.authority!!, newProvider)
        registerProvider(context, HonorBadge.URI_OLD.authority!!, oldProvider)

        assertEquals(HonorBadge.URI_OLD, HonorBadge.resolveBadgeUri(context))
        setManufacturer("HONOR")
        assertTrue(HonorBadge.setBadge(context, 7))
        assertEquals(0, newProvider.callCount)
        assertEquals(1, oldProvider.callCount)
        assertEquals(ChangeBadgeCall.METHOD, oldProvider.lastMethod)
        assertEquals(
            context.packageName,
            oldProvider.lastExtras?.getString(ChangeBadgeCall.KEY_PACKAGE)
        )
        assertEquals(
            TestLauncherActivity::class.java.name,
            oldProvider.lastExtras?.getString(ChangeBadgeCall.KEY_CLASS),
        )
        assertEquals(7, oldProvider.lastExtras?.getInt(ChangeBadgeCall.KEY_BADGE_NUMBER))
    }

    @Test
    @Config(sdk = [Build.VERSION_CODES.P])
    fun huaweiChangeBadgeSendsOfficialExtras() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val provider = RecordingBadgeProvider()
        registerProvider(context, HuaweiBadge.CONTENT_URI.authority!!, provider)

        setManufacturer("HUAWEI")
        assertTrue(HuaweiBadge.setBadge(context, 5))
        assertEquals(ChangeBadgeCall.METHOD, provider.lastMethod)
        assertEquals(
            context.packageName,
            provider.lastExtras?.getString(ChangeBadgeCall.KEY_PACKAGE)
        )
        assertEquals(
            TestLauncherActivity::class.java.name,
            provider.lastExtras?.getString(ChangeBadgeCall.KEY_CLASS),
        )
        assertEquals(5, provider.lastExtras?.getInt(ChangeBadgeCall.KEY_BADGE_NUMBER))
    }

    @Test
    fun oppoClearCountIsMinusOne() {
        assertEquals(-1, OppoBadge.launcherCount(0))
        assertEquals(4, OppoBadge.launcherCount(4))
    }

    @Test
    @Config(sdk = [Build.VERSION_CODES.P])
    fun oppoProviderReceivesMinusOneWhenClearing() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val provider = RecordingBadgeProvider()
        registerProvider(context, OppoBadge.CONTENT_URI.authority!!, provider)

        setManufacturer("OPPO")
        assertTrue(OppoBadge.setBadge(context, 0))
        assertEquals(OppoBadge.PROVIDER_METHOD, provider.lastMethod)
        assertEquals(-1, provider.lastExtras?.getInt(OppoBadge.EXTRA_APP_BADGE_COUNT))
    }

    @Test
    @Config(sdk = [Build.VERSION_CODES.P])
    fun vivoBroadcastContainsOfficialExtras() {
        val app = ApplicationProvider.getApplicationContext<Application>()
        setManufacturer("vivo")
        assertTrue(VivoBadge.setBadge(app, 10))

        val intent =
            Shadows.shadowOf(app).broadcastIntents.firstOrNull {
                it.action == VivoBadge.INTENT_ACTION
            }
        assertNotNull(intent)
        assertEquals(app.packageName, intent!!.getStringExtra(VivoBadge.EXTRA_PACKAGE_NAME))
        assertEquals(
            TestLauncherActivity::class.java.name,
            intent.getStringExtra(VivoBadge.EXTRA_CLASS_NAME),
        )
        assertEquals(10, intent.getIntExtra(VivoBadge.EXTRA_NOTIFICATION_NUM, -1))
    }

    @Test
    @Config(sdk = [Build.VERSION_CODES.P])
    fun meizuProviderUsesBadgeNumberKey() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val provider = RecordingBadgeProvider()
        registerProvider(context, MeizuBadge.CONTENT_URI.authority!!, provider)

        setManufacturer("Meizu")
        assertTrue(MeizuBadge.setBadge(context, 3))
        assertEquals(ChangeBadgeCall.METHOD, provider.lastMethod)
        assertEquals(3, provider.lastExtras?.getInt(MeizuBadge.KEY_BADGE_NUMBER))
        assertEquals(
            context.packageName,
            provider.lastExtras?.getString(ChangeBadgeCall.KEY_PACKAGE)
        )
    }

    @Test
    @Config(sdk = [Build.VERSION_CODES.O])
    fun xiaomiZeroCountCancelsNotification() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        setManufacturer("Xiaomi")
        assertTrue(XiaomiBadge.setBadge(context, 6))

        val manager = context.getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        assertEquals(1, Shadows.shadowOf(manager).allNotifications.size)

        assertTrue(XiaomiBadge.setBadge(context, 0))
        assertTrue(Shadows.shadowOf(manager).allNotifications.isEmpty())
    }

    private fun setManufacturer(value: String) {
        ReflectionHelpers.setStaticField(Build::class.java, "MANUFACTURER", value)
    }

    private fun registerProvider(context: Context, authority: String, provider: ContentProvider) {
        val info =
            ProviderInfo().apply {
                this.authority = authority
                packageName = context.packageName
                name = provider.javaClass.name
                exported = true
            }
        provider.attachInfo(context, info)
        ShadowContentResolver.registerProviderInternal(authority, provider)
    }
}
