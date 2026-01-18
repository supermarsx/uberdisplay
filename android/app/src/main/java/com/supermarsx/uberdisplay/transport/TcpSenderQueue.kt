package com.supermarsx.uberdisplay.transport

import com.supermarsx.uberdisplay.Diagnostics
import java.util.concurrent.ConcurrentLinkedQueue

class TcpSenderQueue {
    private val queue = ConcurrentLinkedQueue<ByteArray>()

    fun enqueue(bytes: ByteArray) {
        queue.add(bytes)
        Diagnostics.logInfo("tcp_send_enqueued size=${bytes.size}")
    }

    fun drain(): List<ByteArray> {
        val items = mutableListOf<ByteArray>()
        while (true) {
            val item = queue.poll() ?: break
            items.add(item)
        }
        return items
    }
}
