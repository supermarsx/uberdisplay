package com.supermarsx.uberdisplay.session

import com.supermarsx.uberdisplay.Diagnostics

class SessionManager {
    private var active = false

    fun start() {
        if (active) return
        active = true
        Diagnostics.logInfo("session_start")
    }

    fun stop() {
        if (!active) return
        active = false
        Diagnostics.logInfo("session_stop")
    }

    fun isActive(): Boolean = active
}
