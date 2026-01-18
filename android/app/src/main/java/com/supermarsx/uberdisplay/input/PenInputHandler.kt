package com.supermarsx.uberdisplay.input

import android.view.MotionEvent
import android.view.View

class PenInputHandler(
    private val sender: InputSender
) : View.OnGenericMotionListener {
    override fun onGenericMotion(v: View?, event: MotionEvent?): Boolean {
        if (event == null) return false
        val tool = event.getToolType(0)
        if (tool == MotionEvent.TOOL_TYPE_STYLUS || tool == MotionEvent.TOOL_TYPE_ERASER) {
            val width = v?.width ?: 0
            val height = v?.height ?: 0
            sender.sendPen(event, width, height)
            return true
        }
        return false
    }
}
