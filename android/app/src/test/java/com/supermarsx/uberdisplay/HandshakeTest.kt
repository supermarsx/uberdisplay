package com.supermarsx.uberdisplay

import com.supermarsx.uberdisplay.protocol.Handshake
import com.supermarsx.uberdisplay.protocol.ProtocolConstants
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class HandshakeTest {
    @Test
    fun parsesHandshakeVersion() {
        val version = Handshake.parseVersion("${ProtocolConstants.HANDSHAKE_BASE}004")
        assertEquals(4, version)
    }

    @Test
    fun rejectsInvalidHandshake() {
        assertNull(Handshake.parseVersion("BAD_000"))
    }

    @Test
    fun buildsHelloString() {
        val hello = Handshake.buildHello(3)
        assertEquals("${ProtocolConstants.HANDSHAKE_BASE}003\u0000", hello)
    }
}
