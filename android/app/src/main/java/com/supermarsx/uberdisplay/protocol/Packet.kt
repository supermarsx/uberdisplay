package com.supermarsx.uberdisplay.protocol

sealed class Packet {
    data class State(val code: Int) : Packet()
    data class Configure(val width: Int, val height: Int, val encoderId: Int) : Packet()
    data class Frame(val data: ByteArray) : Packet()
    data class FrameDone(val encoderId: Int) : Packet()
    data class Touch(val points: Int) : Packet()
    data class Pen(val pressure: Int) : Packet()
    data class Keyboard(val keyCode: Int, val down: Boolean) : Packet()
    data class Command(val commandId: Int) : Packet()
    data class Error(val code: Int) : Packet()
}
