package com.kurban.xue_hua_app_badge.vendor

import android.content.Context
import android.net.Uri

internal object HuaweiBadge : VendorBadge {
    val CONTENT_URI: Uri = Uri.parse("content://com.huawei.android.launcher.settings/badge/")

    override fun matches(manufacturer: String): Boolean = manufacturer.matchesManufacturer("HUAWEI")

    override fun setBadge(context: Context, count: Int): Boolean {
        return ChangeBadgeCall.apply(context, CONTENT_URI, count)
    }
}
