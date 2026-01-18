package com.supermarsx.uberdisplay.transport

import com.supermarsx.uberdisplay.Diagnostics
import com.supermarsx.uberdisplay.protocol.StreamChunkWriter
import java.io.OutputStream

class TcpSenderLoop(private val queue: TcpSenderQueue) {
    @Volatile
    private var running = false
    private var thread: Thread? = null
    private val chunkWriter = StreamChunkWriter()

    fun start(output: OutputStream) {
        if (running) return
        running = true
        thread = Thread {
            while (running) {
                val items = queue.drain()
                for (item in items) {
                    try {
                        val chunks = chunkWriter.wrapChunks(0, item)
                        for (chunk in chunks) {
                            output.write(chunk)
                        }
                        output.flush()
                        Diagnostics.logInfo("tcp_send size=${item.size} chunks=${chunks.size}")
                    } catch (_: Exception) {
                        running = false
                        return@Thread
                    }
                }
                try {
                    Thread.sleep(50)
                } catch (_: InterruptedException) {
                    return@Thread
                }
            }
        }.also { it.start() }
    }

    fun stop() {
        running = false
        thread?.interrupt()
        thread = null
    }
}
