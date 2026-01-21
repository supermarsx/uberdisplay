package com.supermarsx.uberdisplay

import com.supermarsx.uberdisplay.protocol.ProtocolDataTypes
import com.supermarsx.uberdisplay.protocol.StreamChunkParser
import org.junit.Assert.assertTrue
import org.junit.Test
import java.nio.ByteBuffer
import java.nio.ByteOrder

class StreamChunkParserTest {
    @Test
    fun acceptsChunkWithConfigurePacket() {
        val packet = ByteBuffer.allocate(1 + 12).order(ByteOrder.LITTLE_ENDIAN)
        packet.put(ProtocolDataTypes.CONFIGURE.toByte())
        packet.putInt(800)
        packet.putInt(600)
        packet.putInt(1)

        val framed = ByteBuffer.allocate(4 + packet.array().size).order(ByteOrder.LITTLE_ENDIAN)
        framed.putInt(packet.array().size)
        framed.put(packet.array())

        StreamChunkParser().appendChunk(0, framed.array())
    }

    @Test
    fun acceptsMultiplePacketsInOneChunk() {
        val packetA = byteArrayOf(ProtocolDataTypes.STATE.toByte(), 1)
        val packetB = byteArrayOf(ProtocolDataTypes.ERROR.toByte(), 2)

        val framed = ByteBuffer.allocate(4 + packetA.size + 4 + packetB.size)
            .order(ByteOrder.LITTLE_ENDIAN)
        framed.putInt(packetA.size)
        framed.put(packetA)
        framed.putInt(packetB.size)
        framed.put(packetB)

        StreamChunkParser().appendChunk(1, framed.array())
    }

    @Test
    fun enqueuesFrameDoneAfterFrame() {
        val configure = ByteBuffer.allocate(1 + 12).order(ByteOrder.LITTLE_ENDIAN)
        configure.put(ProtocolDataTypes.CONFIGURE.toByte())
        configure.putInt(800)
        configure.putInt(600)
        configure.putInt(5)

        val frame = byteArrayOf(ProtocolDataTypes.FRAME.toByte(), 0, 1)

        val framed = ByteBuffer.allocate(4 + configure.array().size + 4 + frame.size)
            .order(ByteOrder.LITTLE_ENDIAN)
        framed.putInt(configure.array().size)
        framed.put(configure.array())
        framed.putInt(frame.size)
        framed.put(frame)

        val queue = com.supermarsx.uberdisplay.transport.TransportOutbox.tcpQueue
        queue.drain()
        StreamChunkParser().appendChunk(0, framed.array())
        val after = queue.size()
        assertTrue(after >= 1)
    }

    @Test
    fun toleratesIncompleteChunk() {
        val parser = StreamChunkParser()
        parser.appendChunk(0, byteArrayOf(0, 0, 0))
    }
}
