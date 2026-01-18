package com.supermarsx.uberdisplay.input

import android.view.MotionEvent

interface InputSender {
    fun sendTouch(event: MotionEvent)
    fun sendPen(event: MotionEvent)
    fun sendKey(keyCode: Int, down: Boolean)
}
