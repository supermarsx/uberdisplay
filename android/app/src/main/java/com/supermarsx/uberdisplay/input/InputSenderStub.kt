package com.supermarsx.uberdisplay.input

import android.util.Log
import android.view.MotionEvent

class InputSenderStub : InputSender {
    override fun sendTouch(event: MotionEvent) {
        Log.d("InputSenderStub", "touch action=${event.actionMasked} pointers=${event.pointerCount}")
    }

    override fun sendPen(event: MotionEvent) {
        Log.d("InputSenderStub", "pen action=${event.actionMasked}")
    }

    override fun sendKey(keyCode: Int, down: Boolean) {
        Log.d("InputSenderStub", "key code=$keyCode down=$down")
    }
}
