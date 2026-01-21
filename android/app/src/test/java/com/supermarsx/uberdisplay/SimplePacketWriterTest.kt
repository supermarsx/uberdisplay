package com.supermarsx.uberdisplay

import com.supermarsx.uberdisplay.protocol.Packet
import com.supermarsx.uberdisplay.protocol.ProtocolDataTypes
import com.supermarsx.uberdisplay.protocol.SimplePacketWriter
import org.junit.Assert.assertEquals
import org.junit.Test

class SimplePacketWriterTest {
    @Test
    fun writesTouchPacket() {
        val point = Packet.TouchPoint(pointerId = 1, down = true, x = 100, y = 200, size = 50)
        val bytes = SimplePacketWriter().write(Packet.Touch(points = listOf(point)))
        assertEquals(10, bytes.size)
        assertEquals(ProtocolDataTypes.TOUCH.toByte(), bytes[0])
        assertEquals(1.toByte(), bytes[1])
    }

    @Test
    fun writesMultiPointTouchPacket() {
        val points = listOf(
            Packet.TouchPoint(pointerId = 1, down = true, x = 100, y = 200, size = 50),
            Packet.TouchPoint(pointerId = 2, down = true, x = 300, y = 400, size = 60)
        )
        val bytes = SimplePacketWriter().write(Packet.Touch(points = points))
        assertEquals(2.toByte(), bytes[1])
    }

    @Test
    fun writesPenPacket() {
        val bytes = SimplePacketWriter().write(
            Packet.Pen(flags = 1, x = 10, y = 20, pressure = 512, rotation = 0, tilt = 0)
        )
        assertEquals(11, bytes.size)
        assertEquals(ProtocolDataTypes.PEN.toByte(), bytes[0])
        assertEquals(1.toByte(), bytes[1])
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

    @Test
    fun writesInputKeyPacket() {
        val bytes = SimplePacketWriter().write(Packet.InputKey(down = true, buttonIndex = 2, actionId = 77))
        assertEquals(7, bytes.size)
        assertEquals(ProtocolDataTypes.INPUT_KEY.toByte(), bytes[0])
    }

    @Test
    fun writesInputConfigPacket() {
        val bytes = SimplePacketWriter().write(Packet.InputConfig(buttonFunction = 123))
        assertEquals(5, bytes.size)
        assertEquals(ProtocolDataTypes.INPUT_CONFIG.toByte(), bytes[0])
    }
}
