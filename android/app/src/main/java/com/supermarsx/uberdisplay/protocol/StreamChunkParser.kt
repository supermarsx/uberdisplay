package com.supermarsx.uberdisplay.protocol

import com.supermarsx.uberdisplay.Diagnostics

class StreamChunkParser {
    private val buffers = mutableMapOf<Int, StreamBuffer>()
    private val reader = SimplePacketReader()

    fun appendChunk(streamId: Int, chunk: ByteArray) {
        val buffer = buffers.getOrPut(streamId) { StreamBuffer() }
        buffer.append(chunk)
        while (true) {
            val packetBytes = buffer.readPacket() ?: break
            val packet = reader.read(packetBytes)
            if (packet != null) {
                com.supermarsx.uberdisplay.transport.TransportStatus.tcpPacketsIn += 1
                Diagnostics.logInfo("stream_packet stream=$streamId type=${packet::class.simpleName}")
            }
        }
    }
}
