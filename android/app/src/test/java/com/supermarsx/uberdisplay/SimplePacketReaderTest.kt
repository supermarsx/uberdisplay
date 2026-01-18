package com.supermarsx.uberdisplay

import com.supermarsx.uberdisplay.protocol.Packet
import com.supermarsx.uberdisplay.protocol.ProtocolDataTypes
import com.supermarsx.uberdisplay.protocol.SimplePacketReader
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test
import java.nio.ByteBuffer
import java.nio.ByteOrder

class SimplePacketReaderTest {
    @Test
    fun parsesConfigurePacket() {
        val buffer = ByteBuffer.allocate(1 + 12).order(ByteOrder.LITTLE_ENDIAN)
        buffer.put(ProtocolDataTypes.CONFIGURE.toByte())
        buffer.putInt(1920)
        buffer.putInt(1080)
        buffer.putInt(7)

        val packet = SimplePacketReader().read(buffer.array())
        assertTrue(packet is Packet.Configure)
        val configure = packet as Packet.Configure
        assertEquals(1920, configure.width)
        assertEquals(1080, configure.height)
        assertEquals(7, configure.encoderId)
    }
}
