package com.supermarsx.uberdisplay.transport

class AoapTransportStub : Transport {
    private var running = false

    override fun start() {
        running = true
    }

    override fun stop() {
        running = false
    }

    override fun isRunning(): Boolean = running
}
