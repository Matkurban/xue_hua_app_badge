package com.kurban.xue_hua_app_badge.vendor

import android.content.Context
import android.net.Uri
import androidx.core.net.toUri

internal object HuaweiBadge : VendorBadge {
    val CONTENT_URI: Uri = "content://com.huawei.android.launcher.settings/badge/".toUri()

    override fun matches(manufacturer: String): Boolean = manufacturer.matchesManufacturer("HUAWEI")

    override fun setBadge(context: Context, count: Int): Boolean {
        return ChangeBadgeCall.apply(context, CONTENT_URI, count)
    }
}
