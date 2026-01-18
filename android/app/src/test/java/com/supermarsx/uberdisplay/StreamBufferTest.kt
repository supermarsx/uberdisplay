package com.supermarsx.uberdisplay

import com.supermarsx.uberdisplay.protocol.StreamBuffer
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test
import java.nio.ByteBuffer
import java.nio.ByteOrder

class StreamBufferTest {
    @Test
    fun readsPacketWithLengthPrefix() {
        val payload = byteArrayOf(1, 2, 3, 4)
        val buffer = ByteBuffer.allocate(4 + payload.size).order(ByteOrder.LITTLE_ENDIAN)
        buffer.putInt(payload.size)
        buffer.put(payload)

        val stream = StreamBuffer()
        stream.append(buffer.array())
        val packet = stream.readPacket()

        assertArrayEquals(payload, packet)
        assertEquals(0, stream.size())
    }

    @Test
    fun returnsNullWhenIncomplete() {
        val stream = StreamBuffer()
        stream.append(byteArrayOf(1, 0))
        assertNull(stream.readPacket())
    }

    @Test
    fun dropsZeroLengthPacket() {
        val stream = StreamBuffer()
        stream.append(byteArrayOf(0, 0, 0, 0))
        assertNull(stream.readPacket())
        assertEquals(0, stream.size())
    }

    @Test
    fun readsMultiplePacketsSequentially() {
        val payloadA = byteArrayOf(1, 1)
        val payloadB = byteArrayOf(2, 2, 2)
        val buffer = ByteBuffer.allocate(4 + payloadA.size + 4 + payloadB.size)
            .order(ByteOrder.LITTLE_ENDIAN)
        buffer.putInt(payloadA.size)
        buffer.put(payloadA)
        buffer.putInt(payloadB.size)
        buffer.put(payloadB)

        val stream = StreamBuffer()
        stream.append(buffer.array())
        val first = stream.readPacket()
        val second = stream.readPacket()
        assertArrayEquals(payloadA, first)
        assertArrayEquals(payloadB, second)
    }
}
