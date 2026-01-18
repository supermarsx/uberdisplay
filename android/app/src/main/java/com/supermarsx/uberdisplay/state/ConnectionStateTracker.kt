package com.supermarsx.uberdisplay.state

import com.supermarsx.uberdisplay.ConnectionState

class ConnectionStateTracker(initial: ConnectionState = ConnectionState.IDLE) {
    var state: ConnectionState = initial
        private set

    fun canTransition(to: ConnectionState): Boolean {
        if (state == to) return true
        return when (state) {
            ConnectionState.IDLE -> to == ConnectionState.WAITING || to == ConnectionState.ERROR
            ConnectionState.WAITING -> to == ConnectionState.CONNECTED || to == ConnectionState.ERROR
            ConnectionState.CONNECTED -> to == ConnectionState.IDLE || to == ConnectionState.ERROR
            ConnectionState.ERROR -> to == ConnectionState.IDLE
        }
    }

    fun transition(to: ConnectionState): Boolean {
        if (!canTransition(to)) return false
        state = to
        return true
    }
}
