package com.supermarsx.uberdisplay.protocol

interface PacketWriter {
    fun write(packet: Packet): ByteArray
}
