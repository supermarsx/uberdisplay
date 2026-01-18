package com.supermarsx.uberdisplay

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class RootModuleStatusTest {
    @Test
    fun parseHandshake_success() {
        val handshake = RootModuleStatus.parseHandshake("OK 1 caps=0x0000000F", "PONG")
        assertTrue(handshake.ok)
        assertEquals(15L, handshake.caps)
    }

    @Test
    fun parseHandshake_failure() {
        val handshake = RootModuleStatus.parseHandshake("ERR", "PONG")
        assertFalse(handshake.ok)
        assertEquals(0L, handshake.caps)
    }

    @Test
    fun parseHandshake_missingCaps() {
        val handshake = RootModuleStatus.parseHandshake("OK 1", "PONG")
        assertTrue(handshake.ok)
        assertEquals(0L, handshake.caps)
    }
}
