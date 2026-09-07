package com.kurban.xue_hua_app_badge.vendor

import android.content.ContentProviderClient
import android.content.Context
import android.content.Intent
import android.net.Uri
import android.os.Build
import android.os.Bundle

internal object VivoBadge : VendorBadge {
    const val INTENT_ACTION = "launcher.action.CHANGE_APPLICATION_NOTIFICATION_NUM"
    const val EXTRA_PACKAGE_NAME = "packageName"
    const val EXTRA_CLASS_NAME = "className"
    const val EXTRA_NOTIFICATION_NUM = "notificationNum"
    val ORIGIN_OS_URI: Uri =
        Uri.parse("content://com.vivo.abe.provider.launcher.notification.num")

    private const val FLAG_RECEIVER_INCLUDE_BACKGROUND = 0x01000000

    override fun matches(manufacturer: String): Boolean =
        manufacturer.matchesManufacturer("vivo", "iQOO")

    override fun setBadge(context: Context, count: Int): Boolean {
        val broadcastApplied = sendOfficialBroadcast(context, count)
        val originOsApplied = applyOriginOsProvider(context, count)
        return broadcastApplied || originOsApplied
    }

    private fun sendOfficialBroadcast(context: Context, count: Int): Boolean {
        val className = LauncherComponent.className(context) ?: return false
        return try {
            val intent =
                Intent(INTENT_ACTION).apply {
                    putExtra(EXTRA_PACKAGE_NAME, context.packageName)
                    putExtra(EXTRA_CLASS_NAME, className)
                    putExtra(EXTRA_NOTIFICATION_NUM, count)
                    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                        addFlags(FLAG_RECEIVER_INCLUDE_BACKGROUND)
                    }
                }
            context.sendBroadcast(intent)
            true
        } catch (_: Exception) {
            false
        }
    }

    private fun applyOriginOsProvider(context: Context, count: Int): Boolean {
        val className = LauncherComponent.className(context) ?: return false
        val extras =
            Bundle().apply {
                putString(ChangeBadgeCall.KEY_PACKAGE, context.packageName)
                putString(ChangeBadgeCall.KEY_CLASS, className)
                putInt(ChangeBadgeCall.KEY_BADGE_NUMBER, count)
            }
        var client: ContentProviderClient? = null
        return try {
            client = context.contentResolver.acquireUnstableContentProviderClient(ORIGIN_OS_URI)
            if (client == null) {
                return false
            }
            client.call(ChangeBadgeCall.METHOD, null, extras)
            true
        } catch (_: Exception) {
            false
        } finally {
            closeClient(client)
        }
    }

    private fun closeClient(client: ContentProviderClient?) {
        if (client == null) {
            return
        }
        try {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
                client.close()
            } else {
                @Suppress("DEPRECATION")
                client.release()
            }
        } catch (_: Exception) {
        }
    }
}
