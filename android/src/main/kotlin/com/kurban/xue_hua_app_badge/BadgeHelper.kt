package com.kurban.xue_hua_app_badge

import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
import android.os.Build
import androidx.annotation.Keep
import androidx.core.app.NotificationCompat
import androidx.core.app.NotificationManagerCompat
import me.leolin.shortcutbadger.ShortcutBadger

@Keep
object BadgeHelper {
    private const val CHANNEL_ID = "xue_hua_app_badge_silent"
    private const val NOTIFICATION_ID = 0x5875

    @JvmStatic
    fun applyBadge(context: Context, count: Int): Boolean {
        val safeCount = count.coerceAtLeast(0)
        var applied: Boolean

        applied = try {
            ShortcutBadger.applyCount(context, safeCount)
        } catch (_: Exception) {
            false
        }

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val notificationApplied = applyNotificationFallback(context, safeCount)
            applied = applied || notificationApplied
        }

        return applied
    }

    private fun applyNotificationFallback(context: Context, count: Int): Boolean {
        return try {
            val manager =
                context.getSystemService(Context.NOTIFICATION_SERVICE) as? NotificationManager
                    ?: return false

            if (!NotificationManagerCompat.from(context).areNotificationsEnabled()) {
                return false
            }

            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                val channel =
                    NotificationChannel(
                        CHANNEL_ID,
                        "App Badge",
                        NotificationManager.IMPORTANCE_MIN,
                    ).apply {
                        setShowBadge(true)
                        enableLights(false)
                        enableVibration(false)
                        setSound(null, null)
                    }
                manager.createNotificationChannel(channel)
            }

            if (count <= 0) {
                manager.cancel(NOTIFICATION_ID)
                return true
            }

            val notification =
                NotificationCompat.Builder(context, CHANNEL_ID)
                    .setSmallIcon(resolveSmallIcon(context))
                    .setContentTitle("")
                    .setContentText("")
                    .setNumber(count)
                    .setShowWhen(false)
                    .setOnlyAlertOnce(true)
                    .setSilent(true)
                    .setPriority(NotificationCompat.PRIORITY_MIN)
                    .build()

            manager.notify(NOTIFICATION_ID, notification)
            true
        } catch (_: Exception) {
            false
        }
    }

    private fun resolveSmallIcon(context: Context): Int {
        val iconId = context.applicationInfo.icon
        return if (iconId != 0) iconId else android.R.drawable.sym_def_app_icon
    }
}
