package com.supermarsx.uberdisplay.transport

class TransportListener(
    private val tcpListener: TcpListenerStub = TcpListenerStub(),
    private val aoapListener: AoapListenerStub = AoapListenerStub()
) {
    fun start() {
        tcpListener.start()
        aoapListener.start()
        TransportStatus.tcpState = TransportStatus.State.LISTENING
        TransportStatus.aoapState = TransportStatus.State.WAITING
    }

    fun stop() {
        tcpListener.stop()
        aoapListener.stop()
        TransportStatus.tcpState = TransportStatus.State.STOPPED
        TransportStatus.aoapState = TransportStatus.State.STOPPED
    }

    fun isRunning(): Boolean {
        return tcpListener.isRunning() || aoapListener.isRunning()
    }
}
