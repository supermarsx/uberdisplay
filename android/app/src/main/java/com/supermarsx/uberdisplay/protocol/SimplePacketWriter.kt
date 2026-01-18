package com.supermarsx.uberdisplay.protocol

import java.nio.ByteBuffer
import java.nio.ByteOrder

class SimplePacketWriter : PacketWriter {
    override fun write(packet: Packet): ByteArray {
        return when (packet) {
            is Packet.Touch -> writeTouch(packet)
            is Packet.Pen -> writePen(packet)
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
}
