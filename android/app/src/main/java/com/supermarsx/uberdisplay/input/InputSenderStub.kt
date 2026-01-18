package com.supermarsx.uberdisplay.input

import android.util.Log
import android.view.MotionEvent
import com.supermarsx.uberdisplay.protocol.Packet
import com.supermarsx.uberdisplay.protocol.SimplePacketWriter
import com.supermarsx.uberdisplay.transport.TransportOutbox

class InputSenderStub : InputSender {
    private val writer = SimplePacketWriter()
    private val senderQueue = TransportOutbox.tcpQueue

    override fun sendTouch(event: MotionEvent) {
        val packet = Packet.Touch(points = event.pointerCount)
        val bytes = writer.write(packet)
        senderQueue.enqueue(bytes)
        Log.d(
            "InputSenderStub",
            "touch action=${event.actionMasked} pointers=${event.pointerCount} bytes=${bytes.size}"
        )
    }

    override fun sendPen(event: MotionEvent) {
        val pressure = (event.pressure * 1024).toInt()
        val packet = Packet.Pen(pressure = pressure)
        val bytes = writer.write(packet)
        senderQueue.enqueue(bytes)
        Log.d(
            "InputSenderStub",
            "pen action=${event.actionMasked} pressure=$pressure bytes=${bytes.size}"
        )
    }

    override fun sendKey(keyCode: Int, down: Boolean) {
        Log.d("InputSenderStub", "key code=$keyCode down=$down")
    }
}
