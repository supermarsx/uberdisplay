package com.supermarsx.uberdisplay.transport

object TransportStatus {
    enum class State {
        STOPPED,
        LISTENING,
        WAITING
    }

    @Volatile
    var tcpState: State = State.STOPPED

    @Volatile
    var aoapState: State = State.STOPPED

    @Volatile
    var tcpConnections: Int = 0

    @Volatile
    var lastTcpConnectionAt: Long = 0

    @Volatile
    var tcpPacketsIn: Int = 0

    @Volatile
    var tcpPacketsOut: Int = 0
}
