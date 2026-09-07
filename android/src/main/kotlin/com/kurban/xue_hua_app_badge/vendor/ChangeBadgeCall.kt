package com.kurban.xue_hua_app_badge.vendor

import android.content.Context
import android.net.Uri
import android.os.Bundle

internal object ChangeBadgeCall {
    const val METHOD = "change_badge"
    const val KEY_PACKAGE = "package"
    const val KEY_CLASS = "class"
    const val KEY_BADGE_NUMBER = "badgenumber"

    fun apply(
        context: Context,
        uri: Uri,
        count: Int,
        countKey: String = KEY_BADGE_NUMBER,
    ): Boolean {
        val className = LauncherComponent.className(context) ?: return false
        val extras =
            Bundle().apply {
                putString(KEY_PACKAGE, context.packageName)
                putString(KEY_CLASS, className)
                putInt(countKey, count)
            }
        return try {
            context.contentResolver.call(uri, METHOD, null, extras)
            true
        } catch (_: Exception) {
            false
        }
    }
}
