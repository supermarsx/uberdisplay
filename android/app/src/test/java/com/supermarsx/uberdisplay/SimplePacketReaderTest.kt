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

    @Test
    fun parsesStatePacket() {
        val bytes = byteArrayOf(ProtocolDataTypes.STATE.toByte(), 5)
        val packet = SimplePacketReader().read(bytes)
        assertTrue(packet is Packet.State)
        val state = packet as Packet.State
        assertEquals(5, state.code)
    }

    @Test
    fun parsesErrorPacket() {
        val bytes = byteArrayOf(ProtocolDataTypes.ERROR.toByte(), 3)
        val packet = SimplePacketReader().read(bytes)
        assertTrue(packet is Packet.Error)
        val error = packet as Packet.Error
        assertEquals(3, error.code)
    }

    @Test
    fun parsesFrameDonePacket() {
        val buffer = ByteBuffer.allocate(1 + 4).order(ByteOrder.LITTLE_ENDIAN)
        buffer.put(ProtocolDataTypes.FRAME_DONE.toByte())
        buffer.putInt(9)
        val packet = SimplePacketReader().read(buffer.array())
        assertTrue(packet is Packet.FrameDone)
        val frameDone = packet as Packet.FrameDone
        assertEquals(9, frameDone.encoderId)
    }
}
