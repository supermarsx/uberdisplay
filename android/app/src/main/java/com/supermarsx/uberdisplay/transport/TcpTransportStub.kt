package com.supermarsx.uberdisplay.transport

class TcpTransportStub(
    private val port: Int = DEFAULT_PORT
) : Transport {
    private var running = false

    override fun start() {
        running = true
    }

    override fun stop() {
        running = false
    }

    override fun isRunning(): Boolean = running

    companion object {
        const val DEFAULT_PORT = 1445
    }
}
