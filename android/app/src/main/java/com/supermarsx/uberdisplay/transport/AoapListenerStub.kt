package com.supermarsx.uberdisplay.transport

class AoapListenerStub {
    private var running = false

    fun start() {
        running = true
        TransportStatus.aoapState = TransportStatus.State.WAITING
    }

    fun stop() {
        running = false
        TransportStatus.aoapState = TransportStatus.State.STOPPED
    }

    fun isRunning(): Boolean = running
}
