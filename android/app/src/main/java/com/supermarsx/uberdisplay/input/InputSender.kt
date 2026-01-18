package com.supermarsx.uberdisplay.input

import android.view.MotionEvent

interface InputSender {
    fun sendTouch(event: MotionEvent, viewWidth: Int, viewHeight: Int)
    fun sendPen(event: MotionEvent, viewWidth: Int, viewHeight: Int)
    fun sendKey(keyCode: Int, down: Boolean)
}
