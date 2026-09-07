package com.kurban.xue_hua_app_badge

import android.content.Context
import android.os.Build
import androidx.annotation.Keep
import com.kurban.xue_hua_app_badge.vendor.VendorBadges
import com.kurban.xue_hua_app_badge.vendor.XiaomiBadge
import me.leolin.shortcutbadger.ShortcutBadger

@Keep
object BadgeHelper {
    @JvmStatic
    fun applyBadge(context: Context, count: Int): Boolean {
        val safeCount = count.coerceAtLeast(0)
        val manufacturer = Build.MANUFACTURER.orEmpty()
        val vendor = VendorBadges.find(manufacturer)

        val vendorApplied =
            if (vendor != null) {
                try {
                    vendor.setBadge(context, safeCount)
                } catch (_: Exception) {
                    false
                }
            } else {
                try {
                    ShortcutBadger.applyCount(context, safeCount)
                } catch (_: Exception) {
                    false
                }
            }

        if (vendor != null && XiaomiBadge.matches(manufacturer)) {
            return vendorApplied
        }

        val notificationApplied =
            Build.VERSION.SDK_INT >= Build.VERSION_CODES.O && BadgeNotifications.applyDotFallback(context, safeCount)

        return vendorApplied || notificationApplied
    }
}
