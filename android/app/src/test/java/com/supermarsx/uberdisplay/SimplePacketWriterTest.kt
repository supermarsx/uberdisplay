package com.supermarsx.uberdisplay

import com.supermarsx.uberdisplay.protocol.Packet
import com.supermarsx.uberdisplay.protocol.ProtocolDataTypes
import com.supermarsx.uberdisplay.protocol.SimplePacketWriter
import org.junit.Assert.assertEquals
import org.junit.Test

class SimplePacketWriterTest {
    @Test
    fun writesTouchPacket() {
        val bytes = SimplePacketWriter().write(Packet.Touch(points = 2))
        assertEquals(2, bytes.size)
        assertEquals(ProtocolDataTypes.TOUCH.toByte(), bytes[0])
        assertEquals(2.toByte(), bytes[1])
    }

    @Test
    fun writesPenPacket() {
        val bytes = SimplePacketWriter().write(Packet.Pen(pressure = 512))
        assertEquals(3, bytes.size)
        assertEquals(ProtocolDataTypes.PEN.toByte(), bytes[0])
    }

    @Test
    fun returnsEmptyForUnsupportedPacket() {
        val bytes = SimplePacketWriter().write(Packet.Error(code = 1))
        assertEquals(0, bytes.size)
    }

    @Test
    fun writesKeyboardPacket() {
        val bytes = SimplePacketWriter().write(Packet.Keyboard(keyCode = 42, down = true))
        assertEquals(6, bytes.size)
        assertEquals(ProtocolDataTypes.KEYBOARD.toByte(), bytes[0])
    }

    @Test
    fun writesCommandPacket() {
        val bytes = SimplePacketWriter().write(Packet.Command(commandId = 99))
        assertEquals(5, bytes.size)
        assertEquals(ProtocolDataTypes.COMMAND.toByte(), bytes[0])
    }

    @Test
    fun writesFrameDonePacket() {
        val bytes = SimplePacketWriter().write(Packet.FrameDone(encoderId = 11))
        assertEquals(5, bytes.size)
        assertEquals(ProtocolDataTypes.FRAME_DONE.toByte(), bytes[0])
    }
}
