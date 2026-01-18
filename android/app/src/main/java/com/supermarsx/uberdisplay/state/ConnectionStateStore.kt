package com.supermarsx.uberdisplay.state

import com.supermarsx.uberdisplay.ConnectionState
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow

class ConnectionStateStore(initial: ConnectionState = ConnectionState.IDLE) {
    private val mutableState = MutableStateFlow(initial)
    val state: StateFlow<ConnectionState> = mutableState

    fun setState(next: ConnectionState) {
        mutableState.value = next
    }
}
