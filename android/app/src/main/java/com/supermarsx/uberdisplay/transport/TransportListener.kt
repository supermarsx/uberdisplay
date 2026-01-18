package com.supermarsx.uberdisplay.transport

class TransportListener(
    private val tcpListener: TcpListenerStub = TcpListenerStub(),
    private val aoapListener: AoapListenerStub = AoapListenerStub()
) {
    fun start() {
        tcpListener.start()
        aoapListener.start()
    }

    fun stop() {
        tcpListener.stop()
        aoapListener.stop()
    }

    fun isRunning(): Boolean {
        return tcpListener.isRunning() || aoapListener.isRunning()
    }
}
