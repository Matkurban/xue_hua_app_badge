package com.kurban.xue_hua_app_badge.vendor

import android.content.Context
import android.net.Uri
import androidx.core.net.toUri

internal object MeizuBadge : VendorBadge {
    const val KEY_BADGE_NUMBER = "badge_number"
    val CONTENT_URI: Uri =
        "content://com.meizu.flyme.launcher.app_extras/badge_extras".toUri()

    override fun matches(manufacturer: String): Boolean = manufacturer.matchesManufacturer("Meizu")

    override fun setBadge(context: Context, count: Int): Boolean {
        return ChangeBadgeCall.apply(context, CONTENT_URI, count, countKey = KEY_BADGE_NUMBER)
    }
}
