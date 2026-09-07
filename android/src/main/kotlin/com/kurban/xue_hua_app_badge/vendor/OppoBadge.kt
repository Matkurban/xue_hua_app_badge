package com.kurban.xue_hua_app_badge.vendor

import android.content.Context
import android.content.Intent
import android.net.Uri
import android.os.Bundle
import androidx.core.net.toUri

internal object OppoBadge : VendorBadge {
    const val INTENT_ACTION = "com.oppo.unsettledevent"
    const val EXTRA_PACKAGE_NAME = "pakeageName"
    const val EXTRA_NUMBER = "number"
    const val EXTRA_UPGRADE_NUMBER = "upgradeNumber"
    const val EXTRA_APP_BADGE_COUNT = "app_badge_count"
    const val PROVIDER_METHOD = "setAppBadgeCount"
    val CONTENT_URI: Uri = "content://com.android.badge/badge".toUri()

    override fun matches(manufacturer: String): Boolean =
        manufacturer.matchesManufacturer("OPPO", "OnePlus", "realme")

    override fun setBadge(context: Context, count: Int): Boolean {
        val launcherCount = launcherCount(count)
        val intent =
            Intent(INTENT_ACTION).apply {
                putExtra(EXTRA_PACKAGE_NAME, context.packageName)
                putExtra(EXTRA_NUMBER, launcherCount)
                putExtra(EXTRA_UPGRADE_NUMBER, launcherCount)
            }
        if (canResolveBroadcast(context, intent)) {
            return try {
                context.sendBroadcast(intent)
                true
            } catch (_: Exception) {
                false
            }
        }
        return try {
            val extras =
                Bundle().apply {
                    putInt(EXTRA_APP_BADGE_COUNT, launcherCount)
                }
            context.contentResolver.call(CONTENT_URI, PROVIDER_METHOD, null, extras)
            true
        } catch (_: Exception) {
            false
        }
    }

    internal fun launcherCount(count: Int): Int = if (count == 0) -1 else count

    private fun canResolveBroadcast(context: Context, intent: Intent): Boolean {
        val receivers = context.packageManager.queryBroadcastReceivers(intent, 0)
        return receivers.isNotEmpty()
    }
}
