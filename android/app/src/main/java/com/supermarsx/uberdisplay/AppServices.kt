package com.supermarsx.uberdisplay

import com.supermarsx.uberdisplay.controller.ConnectionController
import com.supermarsx.uberdisplay.media.DecoderController

object AppServices {
    private var initialized = false
    val connectionController: ConnectionController by lazy { ConnectionController() }
    val decoderController: DecoderController by lazy { DecoderController() }

    fun init(context: android.content.Context) {
        if (initialized) return
        val prefs = androidx.preference.PreferenceManager.getDefaultSharedPreferences(context)
        Diagnostics.setEnabled(prefs.getBoolean("diagnostics_enabled", true))
        initialized = true
    }
}
