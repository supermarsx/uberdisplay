package com.supermarsx.uberdisplay.protocol

import java.nio.ByteBuffer
import java.nio.ByteOrder

class StreamChunkWriter {
    fun wrap(streamId: Int, payload: ByteArray): ByteArray {
        val buffer = ByteBuffer.allocate(3 + payload.size).order(ByteOrder.LITTLE_ENDIAN)
        buffer.put(streamId.toByte())
        buffer.putShort(payload.size.toShort())
        buffer.put(payload)
        return buffer.array()
    }

    fun wrapChunks(streamId: Int, payload: ByteArray): List<ByteArray> {
        val chunks = mutableListOf<ByteArray>()
        var offset = 0
        while (offset < payload.size) {
            val len = minOf(MAX_CHUNK, payload.size - offset)
            val slice = payload.copyOfRange(offset, offset + len)
            chunks.add(wrap(streamId, slice))
            offset += len
        }
        return chunks
    }

    companion object {
        const val MAX_CHUNK = 65535
    }
}
