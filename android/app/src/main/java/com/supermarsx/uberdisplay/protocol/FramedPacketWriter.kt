package com.supermarsx.uberdisplay.protocol

import java.nio.ByteBuffer
import java.nio.ByteOrder

class FramedPacketWriter(private val writer: PacketWriter = SimplePacketWriter()) {
    fun write(packet: Packet): ByteArray {
        val payload = writer.write(packet)
        if (payload.isEmpty()) return payload
        val buffer = ByteBuffer.allocate(4 + payload.size).order(ByteOrder.LITTLE_ENDIAN)
        buffer.putInt(payload.size)
        buffer.put(payload)
        return buffer.array()
    }
}
