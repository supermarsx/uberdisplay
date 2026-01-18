package com.supermarsx.uberdisplay.session

import com.supermarsx.uberdisplay.Diagnostics

class SessionManager {
    private var active = false
    private val heartbeatManager = HeartbeatManager()

    fun start() {
        if (active) return
        active = true
        heartbeatManager.start()
        Diagnostics.logInfo("session_start")
    }

    fun stop() {
        if (!active) return
        active = false
        heartbeatManager.stop()
        Diagnostics.logInfo("session_stop")
    }

    fun isActive(): Boolean = active
}
