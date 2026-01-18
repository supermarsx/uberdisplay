package com.supermarsx.uberdisplay.input

import android.view.MotionEvent
import android.view.View

class TouchInputHandler(
    private val sender: InputSender
) : View.OnTouchListener {
    override fun onTouch(v: View?, event: MotionEvent?): Boolean {
        if (event == null) return false
        val width = v?.width ?: 0
        val height = v?.height ?: 0
        sender.sendTouch(event, width, height)
        return true
    }
}
