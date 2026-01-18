package com.supermarsx.uberdisplay.protocol

import java.nio.ByteBuffer
import java.nio.ByteOrder

class SimplePacketReader : PacketReader {
    override fun read(bytes: ByteArray): Packet? {
        if (bytes.isEmpty()) return null
        return when (bytes[0].toInt()) {
            ProtocolDataTypes.CONFIGURE -> parseConfigure(bytes)
            ProtocolDataTypes.FRAME -> Packet.Frame(bytes.drop(2).toByteArray())
            ProtocolDataTypes.STATE -> parseState(bytes)
            ProtocolDataTypes.ERROR -> parseError(bytes)
            ProtocolDataTypes.FRAME_DONE -> parseFrameDone(bytes)
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

    private fun parseState(bytes: ByteArray): Packet? {
        if (bytes.size < 2) return null
        val code = bytes[1].toInt() and 0xFF
        return Packet.State(code)
    }

    private fun parseError(bytes: ByteArray): Packet? {
        if (bytes.size < 2) return null
        val code = bytes[1].toInt() and 0xFF
        return Packet.Error(code)
    }

    private fun parseFrameDone(bytes: ByteArray): Packet? {
        if (bytes.size < 1 + 4) return null
        val buffer = ByteBuffer.wrap(bytes, 1, bytes.size - 1).order(ByteOrder.LITTLE_ENDIAN)
        val encoderId = buffer.int
        return Packet.FrameDone(encoderId)
    }
}
