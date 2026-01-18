package com.supermarsx.uberdisplay.session

import com.supermarsx.uberdisplay.ConnectionState

class MirrorSessionController {
    var state: ConnectionState = ConnectionState.IDLE
        private set

    fun startSession() {
        state = ConnectionState.CONNECTED
    }

    fun stopSession() {
        state = ConnectionState.IDLE
    }

    fun setError() {
        state = ConnectionState.ERROR
    }
}
