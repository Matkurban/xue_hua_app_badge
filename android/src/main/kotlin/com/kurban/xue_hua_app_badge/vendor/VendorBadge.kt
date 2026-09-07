package com.kurban.xue_hua_app_badge.vendor

import android.content.Context

internal interface VendorBadge {
    fun matches(manufacturer: String): Boolean

    fun setBadge(context: Context, count: Int): Boolean
}

internal object VendorBadges {
    private val vendors: List<VendorBadge> =
        listOf(
            HonorBadge,
            HuaweiBadge,
            XiaomiBadge,
            OppoBadge,
            VivoBadge,
            MeizuBadge,
        )

    fun find(manufacturer: String): VendorBadge? =
        vendors.firstOrNull { it.matches(manufacturer) }
}

internal fun String.matchesManufacturer(vararg names: String): Boolean =
    names.any { equals(it, ignoreCase = true) }
