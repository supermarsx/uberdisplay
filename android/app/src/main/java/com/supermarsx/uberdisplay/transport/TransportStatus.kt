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
}
