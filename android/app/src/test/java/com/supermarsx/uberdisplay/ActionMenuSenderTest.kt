package com.supermarsx.uberdisplay

import com.supermarsx.uberdisplay.actionmenu.ActionMenuItem
import com.supermarsx.uberdisplay.actionmenu.ActionMenuSender
import com.supermarsx.uberdisplay.protocol.FramedPacketWriter
import com.supermarsx.uberdisplay.protocol.Packet
import com.supermarsx.uberdisplay.protocol.ProtocolDataTypes
import org.junit.Assert.assertEquals
import org.junit.Test
import java.nio.ByteBuffer
import java.nio.ByteOrder

class ActionMenuSenderTest {
    @Test
    fun sendTapProducesInputKeyFrames() {
        val writer = FramedPacketWriter()
        val packet = Packet.InputKey(down = true, buttonIndex = 1, actionId = 42)
        val framed = writer.write(packet)
        val length = ByteBuffer.wrap(framed).order(ByteOrder.LITTLE_ENDIAN).int
        assertEquals(framed.size - 4, length)
        assertEquals(ProtocolDataTypes.INPUT_KEY.toByte(), framed[4])
    }

    @Test
    fun sendConfigProducesInputConfigFrames() {
        val writer = FramedPacketWriter()
        val packet = Packet.InputConfig(buttonFunction = 99)
        val framed = writer.write(packet)
        assertEquals(ProtocolDataTypes.INPUT_CONFIG.toByte(), framed[4])
    }

    @Test
    fun senderSupportsTapAndConfig() {
        val sender = ActionMenuSender()
        sender.sendTap(ActionMenuItem(1, "Test", 100))
        sender.sendConfig(ActionMenuItem(1, "Test", 100))
    }
}
