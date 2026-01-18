package com.supermarsx.uberdisplay

import com.supermarsx.uberdisplay.protocol.StreamChunkWriter
import org.junit.Assert.assertEquals
import org.junit.Test

class StreamChunkWriterTest {
    @Test
    fun wrapsChunkWithHeader() {
        val payload = byteArrayOf(1, 2, 3)
        val chunk = StreamChunkWriter().wrap(1, payload)
        assertEquals(6, chunk.size)
        assertEquals(1, chunk[0].toInt())
    }

    @Test
    fun splitsLargePayload() {
        val payload = ByteArray(StreamChunkWriter.MAX_CHUNK + 1) { 0 }
        val chunks = StreamChunkWriter().wrapChunks(0, payload)
        assertEquals(2, chunks.size)
    }
}
