package com.supermarsx.uberdisplay.transport

class AoapListenerStub {
    private var running = false

    fun start() {
        running = true
    }

    fun stop() {
        running = false
    }

    fun isRunning(): Boolean = running
}
