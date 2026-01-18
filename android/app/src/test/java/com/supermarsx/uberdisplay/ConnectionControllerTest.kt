package com.supermarsx.uberdisplay

import com.supermarsx.uberdisplay.controller.ConnectionController
import org.junit.Assert.assertEquals
import org.junit.Test

class ConnectionControllerTest {
    @Test
    fun startTcpMovesToWaiting() {
        val controller = ConnectionController()
        controller.startTcp()
        assertEquals(ConnectionState.WAITING, controller.stateStore().state.value)
        controller.stop()
    }

    @Test
    fun markConnectedMovesToConnected() {
        val controller = ConnectionController()
        controller.markConnected()
        assertEquals(ConnectionState.CONNECTED, controller.stateStore().state.value)
        controller.stop()
    }
}
