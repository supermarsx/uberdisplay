package com.supermarsx.uberdisplay.protocol

import com.supermarsx.uberdisplay.Diagnostics
import com.supermarsx.uberdisplay.AppServices
import com.supermarsx.uberdisplay.transport.TransportOutbox

class StreamChunkParser {
    private val buffers = mutableMapOf<Int, StreamBuffer>()
    private val reader = SimplePacketReader()
    private val framedWriter = FramedPacketWriter()
    private var lastEncoderId: Int? = null

    fun appendChunk(streamId: Int, chunk: ByteArray) {
        val buffer = buffers.getOrPut(streamId) { StreamBuffer() }
        buffer.append(chunk)
        while (true) {
            val packetBytes = buffer.readPacket() ?: break
            val packet = reader.read(packetBytes)
            if (packet != null) {
                com.supermarsx.uberdisplay.transport.TransportStatus.tcpPacketsIn += 1
                Diagnostics.logInfo("stream_packet stream=$streamId type=${packet::class.simpleName}")
                if (packet is Packet.Configure) {
                    lastEncoderId = packet.encoderId
                    AppServices.decoderController.onConfigure(packet)
                }
                if (packet is Packet.Frame) {
                    AppServices.decoderController.onFrame(packet.data)
                    sendFrameDone()
                }
            }
        }
    }

    private fun sendFrameDone() {
        val encoderId = lastEncoderId ?: return
        val bytes = framedWriter.write(Packet.FrameDone(encoderId))
        if (bytes.isNotEmpty()) {
            TransportOutbox.tcpQueue.enqueue(bytes)
        }
    }
}
