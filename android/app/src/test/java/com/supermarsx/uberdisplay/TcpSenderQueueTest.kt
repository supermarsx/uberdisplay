package com.supermarsx.uberdisplay

import com.supermarsx.uberdisplay.transport.TcpSenderQueue
import org.junit.Assert.assertEquals
import org.junit.Test

class TcpSenderQueueTest {
    @Test
    fun drainsQueuedItems() {
        val queue = TcpSenderQueue()
        queue.enqueue(byteArrayOf(1))
        queue.enqueue(byteArrayOf(2, 3))
        val drained = queue.drain()
        assertEquals(2, drained.size)
        assertEquals(0, queue.drain().size)
    }
}
