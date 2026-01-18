package com.supermarsx.uberdisplay

import com.supermarsx.uberdisplay.protocol.FramedPacketWriter
import com.supermarsx.uberdisplay.protocol.Packet
import com.supermarsx.uberdisplay.protocol.ProtocolDataTypes
import org.junit.Assert.assertEquals
import org.junit.Test
import java.nio.ByteBuffer
import java.nio.ByteOrder

class FramedPacketWriterTest {
    @Test
    fun prefixesPayloadLength() {
        val point = Packet.TouchPoint(pointerId = 1, down = true, x = 0, y = 0, size = 0)
        val framed = FramedPacketWriter().write(Packet.Touch(points = listOf(point)))
        val buffer = ByteBuffer.wrap(framed).order(ByteOrder.LITTLE_ENDIAN)
        val len = buffer.int
        assertEquals(framed.size - 4, len)
        assertEquals(ProtocolDataTypes.TOUCH.toByte(), framed[4])
    }
}
