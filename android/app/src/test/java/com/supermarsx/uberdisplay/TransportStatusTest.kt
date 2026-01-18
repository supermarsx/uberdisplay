package com.supermarsx.uberdisplay

import com.supermarsx.uberdisplay.transport.TransportStatus
import org.junit.Assert.assertEquals
import org.junit.Test

class TransportStatusTest {
    @Test
    fun resetCounters() {
        TransportStatus.tcpPacketsIn = 5
        TransportStatus.tcpPacketsOut = 6
        TransportStatus.tcpConnections = 2
        TransportStatus.lastTcpConnectionAt = 100

        TransportStatus.tcpPacketsIn = 0
        TransportStatus.tcpPacketsOut = 0
        TransportStatus.tcpConnections = 0
        TransportStatus.lastTcpConnectionAt = 0

        assertEquals(0, TransportStatus.tcpPacketsIn)
        assertEquals(0, TransportStatus.tcpPacketsOut)
        assertEquals(0, TransportStatus.tcpConnections)
        assertEquals(0, TransportStatus.lastTcpConnectionAt)
    }
}
