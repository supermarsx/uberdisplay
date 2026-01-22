package com.supermarsx.uberdisplay.protocol

sealed class Packet {
    data class State(val code: Int) : Packet()
    data class Configure(
        val width: Int,
        val height: Int,
        val hostWidth: Int,
        val hostHeight: Int,
        val encoderId: Int,
        val codecId: Int? = null,
        val codecProfile: Int = 0,
        val codecLevel: Int = 0,
        val codecFlags: Int = 0
    ) : Packet()
    data class Frame(val data: ByteArray, val timestamp100ns: Long? = null) : Packet()
    data class FrameDone(val encoderId: Int) : Packet()
    data class TouchPoint(
        val pointerId: Int,
        val down: Boolean,
        val x: Int,
        val y: Int,
        val size: Int
    )
    data class Touch(val points: List<TouchPoint>) : Packet()
    data class Pen(
        val flags: Int,
        val x: Int,
        val y: Int,
        val pressure: Int,
        val rotation: Int,
        val tilt: Int
    ) : Packet()
    data class Keyboard(val keyCode: Int, val down: Boolean) : Packet()
    data class Command(val commandId: Int) : Packet()
    data class InputKey(val down: Boolean, val buttonIndex: Int, val actionId: Int) : Packet()
    data class InputConfig(val buttonFunction: Int) : Packet()
    data class Error(val code: Int) : Packet()
    data class Capabilities(val codecMask: Int, val flags: Int) : Packet()
}
