package com.supermarsx.uberdisplay

import android.util.Log

object Diagnostics {
    @Volatile
    private var enabled: Boolean = true

    fun setEnabled(value: Boolean) {
        enabled = value
    }

    fun logInfo(message: String) {
        if (!enabled) return
        Log.i("Diagnostics", message)
    }

    fun logError(message: String, throwable: Throwable? = null) {
        if (!enabled) return
        if (throwable == null) {
            Log.e("Diagnostics", message)
        } else {
            Log.e("Diagnostics", message, throwable)
        }
    }
}
