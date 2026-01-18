package com.supermarsx.uberdisplay.transport

interface Transport {
    fun start()
    fun stop()
    fun isRunning(): Boolean
}
