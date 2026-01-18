package com.supermarsx.uberdisplay.protocol

interface PacketReader {
    fun read(bytes: ByteArray): Packet?
}
