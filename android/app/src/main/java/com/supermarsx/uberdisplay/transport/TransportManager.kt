package com.supermarsx.uberdisplay.transport

class TransportManager(
    private val tcp: TcpTransportStub = TcpTransportStub(),
    private val aoap: AoapTransportStub = AoapTransportStub()
) {
    enum class Mode {
        TCP,
        AOAP
    }

    private var activeMode: Mode = Mode.TCP

    fun setMode(mode: Mode) {
        if (activeMode == mode) return
        stop()
        activeMode = mode
    }

    fun start() {
        when (activeMode) {
            Mode.TCP -> tcp.start()
            Mode.AOAP -> aoap.start()
        }
    }

    fun stop() {
        tcp.stop()
        aoap.stop()
    }

    fun isRunning(): Boolean {
        return tcp.isRunning() || aoap.isRunning()
    }
}
