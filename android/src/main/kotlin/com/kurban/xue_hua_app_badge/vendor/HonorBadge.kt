package com.kurban.xue_hua_app_badge.vendor

import android.content.ContentResolver
import android.content.Context
import android.net.Uri
import androidx.core.net.toUri

internal object HonorBadge : VendorBadge {
    val URI_NEW: Uri = "content://com.hihonor.android.launcher.settings/badge/".toUri()
    val URI_OLD: Uri = "content://com.huawei.android.launcher.settings/badge/".toUri()

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
