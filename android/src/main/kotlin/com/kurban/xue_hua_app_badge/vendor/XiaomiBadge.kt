package com.kurban.xue_hua_app_badge.vendor

import android.app.Notification
import android.app.NotificationManager
import android.content.Context
import androidx.core.app.NotificationCompat
import androidx.core.app.NotificationManagerCompat
import com.kurban.xue_hua_app_badge.BadgeNotifications
import java.lang.reflect.Method

internal object XiaomiBadge : VendorBadge {
    override fun matches(manufacturer: String): Boolean =
        manufacturer.matchesManufacturer("Xiaomi", "Redmi", "POCO")

    override fun setBadge(context: Context, count: Int): Boolean {
        return try {
            val manager =
                context.getSystemService(Context.NOTIFICATION_SERVICE) as? NotificationManager
                    ?: return false

            if (count <= 0) {
                manager.cancel(BadgeNotifications.NOTIFICATION_ID)
                return true
            }

            if (!NotificationManagerCompat.from(context).areNotificationsEnabled()) {
                return false
            }

            BadgeNotifications.ensureChannel(manager)

            val useNotificationNumber = isMiui12OrNewer()
            val builder =
                NotificationCompat.Builder(context, BadgeNotifications.CHANNEL_ID)
                    .setSmallIcon(BadgeNotifications.resolveSmallIcon(context))
                    .setContentTitle("")
                    .setContentText("")
                    .setOngoing(false)
                    .setShowWhen(false)
                    .setOnlyAlertOnce(true)
                    .setSilent(true)
                    .setPriority(NotificationCompat.PRIORITY_MIN)

            if (useNotificationNumber) {
                builder.setNumber(count)
            }

            val notification = builder.build()
            if (!useNotificationNumber) {
                applyLegacyMessageCount(notification, count)
            }

            manager.notify(BadgeNotifications.NOTIFICATION_ID, notification)
            true
        } catch (_: Exception) {
            false
        }
    }

    internal fun isMiui12OrNewer(): Boolean {
        if (systemProperty("ro.mi.os.version.name").isNotEmpty()) {
            return true
        }
        val name = systemProperty("ro.miui.ui.version.name")
        if (name.isEmpty()) {
            return true
        }
        val digits = name.trim().removePrefix("V").removePrefix("v").takeWhile { it.isDigit() }
        val version = digits.toIntOrNull() ?: return true
        return version >= 12
    }

    private fun applyLegacyMessageCount(notification: Notification, count: Int) {
        try {
            val field = notification.javaClass.getDeclaredField("extraNotification")
            field.isAccessible = true
            val extraNotification = field.get(notification) ?: return
            val method: Method =
                extraNotification.javaClass.getDeclaredMethod("setMessageCount", Int::class.javaPrimitiveType)
            method.invoke(extraNotification, count)
        } catch (_: Exception) {
        }
    }

    private fun systemProperty(key: String, default: String = ""): String {
        return try {
            val clazz = Class.forName("android.os.SystemProperties")
            val method = clazz.getMethod("get", String::class.java, String::class.java)
            (method.invoke(null, key, default) as? String) ?: default
        } catch (_: Exception) {
            default
        }
    }
}
