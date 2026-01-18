package com.supermarsx.uberdisplay.transport

class TcpListenerStub(
    private val port: Int = TcpTransportStub.DEFAULT_PORT
) {
    private var running = false

    fun start() {
        running = true
    }

    fun stop() {
        running = false
    }

    fun isRunning(): Boolean = running

    fun getPort(): Int = port
}
