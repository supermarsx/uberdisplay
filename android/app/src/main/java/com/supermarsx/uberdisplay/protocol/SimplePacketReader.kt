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
            ProtocolDataTypes.CAPABILITIES -> parseCapabilities(bytes)
            else -> null
        }
    }

    private fun parseConfigure(bytes: ByteArray): Packet? {
        if (bytes.size < 1 + 12) return null
        val buffer = ByteBuffer.wrap(bytes, 1, bytes.size - 1).order(ByteOrder.LITTLE_ENDIAN)
        val width = buffer.int
        val height = buffer.int
        val hostWidth: Int
        val hostHeight: Int
        val encoderId: Int
        if (buffer.remaining() >= 12) {
            hostWidth = buffer.int
            hostHeight = buffer.int
            encoderId = buffer.int
        } else {
            encoderId = buffer.int
            hostWidth = width
            hostHeight = height
        }

        var codecId: Int? = null
        var codecProfile = 0
        var codecLevel = 0
        var codecFlags = 0
        if (buffer.remaining() >= 4) {
            codecId = buffer.get().toInt() and 0xFF
            codecProfile = buffer.get().toInt() and 0xFF
            codecLevel = buffer.get().toInt() and 0xFF
            codecFlags = buffer.get().toInt() and 0xFF
        }

        return Packet.Configure(
            width = width,
            height = height,
            hostWidth = hostWidth,
            hostHeight = hostHeight,
            encoderId = encoderId,
            codecId = codecId,
            codecProfile = codecProfile,
            codecLevel = codecLevel,
            codecFlags = codecFlags
        )
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

    private fun parseCapabilities(bytes: ByteArray): Packet? {
        if (bytes.size < 1 + 8) return null
        val buffer = ByteBuffer.wrap(bytes, 1, bytes.size - 1).order(ByteOrder.LITTLE_ENDIAN)
        val codecMask = buffer.int
        val flags = buffer.int
        return Packet.Capabilities(codecMask, flags)
    }
}
