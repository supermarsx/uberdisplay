package com.supermarsx.uberdisplay.transport

import com.supermarsx.uberdisplay.Diagnostics
import java.io.OutputStream

class TcpSenderLoop(private val queue: TcpSenderQueue) {
    @Volatile
    private var running = false
    private var thread: Thread? = null

    fun start(output: OutputStream) {
        if (running) return
        running = true
        thread = Thread {
            while (running) {
                val items = queue.drain()
                for (item in items) {
                    try {
                        output.write(item)
                        output.flush()
                        Diagnostics.logInfo("tcp_send size=${item.size}")
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
