package com.supermarsx.uberdisplay

import com.supermarsx.uberdisplay.state.ConnectionStateTracker
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class ConnectionStateTrackerTest {
    @Test
    fun allowsIdleToWaiting() {
        val tracker = ConnectionStateTracker(ConnectionState.IDLE)
        assertTrue(tracker.transition(ConnectionState.WAITING))
    }

    @Test
    fun rejectsConnectedToWaiting() {
        val tracker = ConnectionStateTracker(ConnectionState.CONNECTED)
        assertFalse(tracker.transition(ConnectionState.WAITING))
    }
}
