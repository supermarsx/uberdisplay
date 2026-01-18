package com.supermarsx.uberdisplay.session

import com.supermarsx.uberdisplay.Diagnostics

class HeartbeatManager {
    @Volatile
    private var running = false
    private var thread: Thread? = null

    fun start() {
        if (running) return
        running = true
        thread = Thread {
            while (running) {
                Diagnostics.logInfo("heartbeat")
                try {
                    Thread.sleep(5000)
                } catch (_: InterruptedException) {
                    return@Thread
                }
            }
        }.also { it.start() }
    }

    fun stop() {
        running = false
        thread?.interrupt()
        thread = null
    }
}
