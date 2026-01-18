package com.supermarsx.uberdisplay.input

import android.util.Log
import android.view.MotionEvent
import com.supermarsx.uberdisplay.protocol.FramedPacketWriter
import com.supermarsx.uberdisplay.protocol.Packet
import com.supermarsx.uberdisplay.transport.TransportOutbox

class InputSenderStub : InputSender {
    private val writer = FramedPacketWriter()
    private val senderQueue = TransportOutbox.tcpQueue

    override fun sendTouch(event: MotionEvent, viewWidth: Int, viewHeight: Int) {
        val points = buildTouchPoints(event, viewWidth, viewHeight)
        val packet = Packet.Touch(points = points)
        val bytes = writer.write(packet)
        senderQueue.enqueue(bytes)
        Log.d(
            "InputSenderStub",
            "touch action=${event.actionMasked} pointers=${points.size} bytes=${bytes.size}"
        )
    }

    override fun sendPen(event: MotionEvent, viewWidth: Int, viewHeight: Int) {
        val flags = buildPenFlags(event)
        val x = normalize(event.x, viewWidth)
        val y = normalize(event.y, viewHeight)
        val pressure = (event.pressure.coerceIn(0f, 1f) * 32767).toInt()
        val rotation = (event.getAxisValue(MotionEvent.AXIS_ROTATION) * 32767).toInt()
        val tilt = (event.getAxisValue(MotionEvent.AXIS_TILT) * 32767).toInt()
        val packet = Packet.Pen(
            flags = flags,
            x = x,
            y = y,
            pressure = pressure,
            rotation = rotation,
            tilt = tilt
        )
        val bytes = writer.write(packet)
        senderQueue.enqueue(bytes)
        Log.d(
            "InputSenderStub",
            "pen action=${event.actionMasked} pressure=$pressure bytes=${bytes.size}"
        )
    }

    override fun sendKey(keyCode: Int, down: Boolean) {
        val packet = Packet.Keyboard(keyCode = keyCode, down = down)
        val bytes = writer.write(packet)
        senderQueue.enqueue(bytes)
        Log.d("InputSenderStub", "key code=$keyCode down=$down bytes=${bytes.size}")
    }

    private fun buildTouchPoints(
        event: MotionEvent,
        viewWidth: Int,
        viewHeight: Int
    ): List<Packet.TouchPoint> {
        val action = event.actionMasked
        val actionIndex = event.actionIndex
        val includeAll = action == MotionEvent.ACTION_MOVE || action == MotionEvent.ACTION_CANCEL

        val indices = if (includeAll) {
            (0 until event.pointerCount).toList()
        } else {
            listOf(actionIndex)
        }

        return indices.map { i ->
            val down = when (action) {
                MotionEvent.ACTION_UP,
                MotionEvent.ACTION_POINTER_UP -> i != actionIndex
                MotionEvent.ACTION_CANCEL -> false
                else -> true
            }
            val x = normalize(event.getX(i), viewWidth)
            val y = normalize(event.getY(i), viewHeight)
            val size = (event.getSize(i) * 32767).toInt()
            Packet.TouchPoint(
                pointerId = event.getPointerId(i),
                down = down,
                x = x,
                y = y,
                size = size
            )
        }
    }

    private fun normalize(value: Float, max: Int): Int {
        if (max <= 0) return 0
        val norm = (value / max).coerceIn(0f, 1f)
        return (norm * 32767).toInt()
    }

    private fun buildPenFlags(event: MotionEvent): Int {
        var flags = 0
        val action = event.actionMasked
        val contact = action == MotionEvent.ACTION_DOWN ||
            action == MotionEvent.ACTION_POINTER_DOWN ||
            action == MotionEvent.ACTION_MOVE
        if (contact) flags = flags or 0x01
        if (action == MotionEvent.ACTION_HOVER_MOVE) flags = flags or 0x02
        if (event.buttonState != 0) flags = flags or 0x04
        return flags
    }
}
