package com.supermarsx.uberdisplay.controller

import android.util.Log
import com.supermarsx.uberdisplay.ConnectionState
import com.supermarsx.uberdisplay.Diagnostics
import com.supermarsx.uberdisplay.state.ConnectionStateTracker
import com.supermarsx.uberdisplay.state.ConnectionStateStore
import com.supermarsx.uberdisplay.transport.TransportManager

class ConnectionController(
    private val transportManager: TransportManager = TransportManager(),
    private val tracker: ConnectionStateTracker = ConnectionStateTracker(),
    private val store: ConnectionStateStore = ConnectionStateStore(),
    private val listener: com.supermarsx.uberdisplay.transport.TransportListener =
        com.supermarsx.uberdisplay.transport.TransportListener(),
    private val sessionManager: com.supermarsx.uberdisplay.session.SessionManager =
        com.supermarsx.uberdisplay.session.SessionManager()
) {
    fun stateStore(): ConnectionStateStore = store

    fun startTcp() {
        start(TransportManager.Mode.TCP)
    }

    fun startAoap() {
        start(TransportManager.Mode.AOAP)
    }

    fun stop() {
        transportManager.stop()
        listener.stop()
        sessionManager.stop()
        Diagnostics.logInfo("transport_status tcp=${com.supermarsx.uberdisplay.transport.TransportStatus.tcpState} aoap=${com.supermarsx.uberdisplay.transport.TransportStatus.aoapState}")
        transition(ConnectionState.IDLE)
    }

    fun markConnected() {
        sessionManager.start()
        transition(ConnectionState.CONNECTED)
    }

    fun markError() {
        transition(ConnectionState.ERROR)
    }

    private fun start(mode: TransportManager.Mode) {
        transportManager.setMode(mode)
        listener.start()
        transportManager.start()
        Diagnostics.logInfo("transport_status tcp=${com.supermarsx.uberdisplay.transport.TransportStatus.tcpState} aoap=${com.supermarsx.uberdisplay.transport.TransportStatus.aoapState}")
        transition(ConnectionState.WAITING)
    }

    private fun transition(next: ConnectionState) {
        val prev = tracker.state
        if (tracker.transition(next)) {
            store.setState(next)
            Log.i("ConnectionController", "state $prev -> $next")
            Diagnostics.logInfo("connection_state $prev -> $next")
        } else {
            Log.w("ConnectionController", "invalid transition $prev -> $next")
            Diagnostics.logError("invalid_transition $prev -> $next")
        }
    }
}
