package com.supermarsx.uberdisplay.protocol

import java.nio.ByteBuffer
import java.nio.ByteOrder

class SimplePacketWriter : PacketWriter {
    override fun write(packet: Packet): ByteArray {
        return when (packet) {
            is Packet.Touch -> writeTouch(packet)
            is Packet.Pen -> writePen(packet)
            is Packet.Keyboard -> writeKeyboard(packet)
            is Packet.Command -> writeCommand(packet)
            is Packet.FrameDone -> writeFrameDone(packet)
            else -> ByteArray(0)
        }
    }

    private fun writeTouch(packet: Packet.Touch): ByteArray {
        val buffer = ByteBuffer.allocate(2).order(ByteOrder.LITTLE_ENDIAN)
        buffer.put(ProtocolDataTypes.TOUCH.toByte())
        buffer.put(packet.points.toByte())
        return buffer.array()
    }

    private fun writePen(packet: Packet.Pen): ByteArray {
        val buffer = ByteBuffer.allocate(3).order(ByteOrder.LITTLE_ENDIAN)
        buffer.put(ProtocolDataTypes.PEN.toByte())
        buffer.putShort(packet.pressure.toShort())
        return buffer.array()
    }

    private fun writeKeyboard(packet: Packet.Keyboard): ByteArray {
        val buffer = ByteBuffer.allocate(6).order(ByteOrder.LITTLE_ENDIAN)
        buffer.put(ProtocolDataTypes.KEYBOARD.toByte())
        buffer.put(if (packet.down) 1 else 0)
        buffer.putInt(packet.keyCode)
        return buffer.array()
    }

    private fun writeCommand(packet: Packet.Command): ByteArray {
        val buffer = ByteBuffer.allocate(5).order(ByteOrder.LITTLE_ENDIAN)
        buffer.put(ProtocolDataTypes.COMMAND.toByte())
        buffer.putInt(packet.commandId)
        return buffer.array()
    }

    private fun writeFrameDone(packet: Packet.FrameDone): ByteArray {
        val buffer = ByteBuffer.allocate(5).order(ByteOrder.LITTLE_ENDIAN)
        buffer.put(ProtocolDataTypes.FRAME_DONE.toByte())
        buffer.putInt(packet.encoderId)
        return buffer.array()
    }
}
