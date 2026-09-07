package com.kurban.xue_hua_app_badge

import android.content.ContentProvider
import android.content.ContentValues
import android.database.Cursor
import android.net.Uri
import android.os.Bundle

internal class RecordingBadgeProvider : ContentProvider() {
    var type: String? = "badge"
    var lastMethod: String? = null
    var lastArg: String? = null
    var lastExtras: Bundle? = null
    var lastUri: Uri? = null
    var callCount: Int = 0

    override fun onCreate(): Boolean = true

    override fun getType(uri: Uri): String? = type

    override fun call(method: String, arg: String?, extras: Bundle?): Bundle {
        lastMethod = method
        lastArg = arg
        lastExtras = extras?.let { Bundle(it) }
        callCount += 1
        return Bundle()
    }

    override fun call(authority: String, method: String, arg: String?, extras: Bundle?): Bundle {
        lastUri = Uri.parse("content://$authority")
        return call(method, arg, extras)
    }

    override fun query(
        uri: Uri,
        projection: Array<out String>?,
        selection: String?,
        selectionArgs: Array<out String>?,
        sortOrder: String?,
    ): Cursor? = null

    override fun insert(uri: Uri, values: ContentValues?): Uri? = null

    override fun delete(uri: Uri, selection: String?, selectionArgs: Array<out String>?): Int = 0

    override fun update(
        uri: Uri,
        values: ContentValues?,
        selection: String?,
        selectionArgs: Array<out String>?,
    ): Int = 0
}
