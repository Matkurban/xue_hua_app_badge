package com.kurban.xue_hua_app_badge.vendor

import android.content.ContentResolver
import android.content.Context
import android.net.Uri

internal object HonorBadge : VendorBadge {
    val URI_NEW: Uri = Uri.parse("content://com.hihonor.android.launcher.settings/badge/")
    val URI_OLD: Uri = Uri.parse("content://com.huawei.android.launcher.settings/badge/")

    override fun matches(manufacturer: String): Boolean = manufacturer.matchesManufacturer("HONOR")

    override fun setBadge(context: Context, count: Int): Boolean {
        val uri = resolveBadgeUri(context) ?: return false
        return ChangeBadgeCall.apply(context, uri, count)
    }

    fun resolveBadgeUri(context: Context): Uri? {
        val resolver = context.contentResolver
        val newType = typeOf(resolver, URI_NEW)
        if (newType.isNotEmpty()) {
            return URI_NEW
        }
        val oldType = typeOf(resolver, URI_OLD)
        if (oldType.isNotEmpty()) {
            return URI_OLD
        }
        return null
    }

    private fun typeOf(resolver: ContentResolver, uri: Uri): String {
        return try {
            resolver.getType(uri).orEmpty()
        } catch (_: Exception) {
            ""
        }
    }
}
