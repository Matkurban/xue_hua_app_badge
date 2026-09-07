package com.kurban.xue_hua_app_badge.vendor

import android.content.ComponentName
import android.content.Context

internal object LauncherComponent {
    fun resolve(context: Context): ComponentName? {
        return context.packageManager
            .getLaunchIntentForPackage(context.packageName)
            ?.component
    }

    fun className(context: Context): String? = resolve(context)?.className
}
