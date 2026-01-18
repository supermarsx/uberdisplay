package com.supermarsx.uberdisplay.protocol

import java.nio.ByteBuffer
import java.nio.ByteOrder

class SimplePacketReader : PacketReader {
    override fun read(bytes: ByteArray): Packet? {
        if (bytes.isEmpty()) return null
        return when (bytes[0].toInt()) {
            ProtocolDataTypes.CONFIGURE -> parseConfigure(bytes)
            ProtocolDataTypes.FRAME -> Packet.Frame(bytes.drop(2).toByteArray())
            else -> null
        }
    }

    private fun parseConfigure(bytes: ByteArray): Packet? {
        if (bytes.size < 1 + 4 + 4 + 4) return null
        val buffer = ByteBuffer.wrap(bytes, 1, bytes.size - 1).order(ByteOrder.LITTLE_ENDIAN)
        val width = buffer.int
        val height = buffer.int
        val encoderId = buffer.int
        return Packet.Configure(width, height, encoderId)
    }
}
