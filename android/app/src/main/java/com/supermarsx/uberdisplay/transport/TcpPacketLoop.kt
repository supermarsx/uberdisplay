package com.supermarsx.uberdisplay.transport

import com.supermarsx.uberdisplay.Diagnostics
import com.supermarsx.uberdisplay.protocol.Handshake
import com.supermarsx.uberdisplay.protocol.StreamChunkParser
import java.io.InputStream

class TcpPacketLoop {
    private val chunkParser = StreamChunkParser()
    private var handshakeDone = false

    fun run(input: InputStream) {
        val buffer = ByteArray(4096)
        val pending = mutableListOf<Byte>()
        while (true) {
            val count = input.read(buffer)
            if (count <= 0) return
            for (i in 0 until count) {
                pending.add(buffer[i])
            }
            if (!handshakeDone) {
                parseHandshake(pending)
            }
            if (handshakeDone) {
                parseChunks(pending)
            }
        }
    }

    private fun parseHandshake(pending: MutableList<Byte>) {
        val nullIndex = pending.indexOf(0)
        if (nullIndex == -1) return
        val bytes = ByteArray(nullIndex)
        for (i in 0 until nullIndex) {
            bytes[i] = pending[i]
        }
        val line = bytes.toString(Charsets.UTF_8)
        val version = Handshake.parseVersion(line)
        if (version != null) {
            Diagnostics.logInfo("tcp_handshake version=$version")
            handshakeDone = true
        } else {
            Diagnostics.logError("tcp_handshake invalid")
        }
        pending.subList(0, nullIndex + 1).clear()
    }

    private fun parseChunks(pending: MutableList<Byte>) {
        while (pending.size >= 3) {
            val streamId = pending[0].toInt() and 0xFF
            val chunkLen = (pending[1].toInt() and 0xFF) or
                ((pending[2].toInt() and 0xFF) shl 8)
            if (pending.size < 3 + chunkLen) return
            val chunk = ByteArray(chunkLen)
            for (i in 0 until chunkLen) {
                chunk[i] = pending[3 + i]
            }
            pending.subList(0, 3 + chunkLen).clear()
            Diagnostics.logInfo("tcp_chunk stream=$streamId len=$chunkLen")
            chunkParser.appendChunk(streamId, chunk)
        }
    }
}
