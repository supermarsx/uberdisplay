package com.supermarsx.uberdisplay.protocol

class StreamBuffer {
    private val data = mutableListOf<Byte>()

    fun append(bytes: ByteArray) {
        for (b in bytes) {
            data.add(b)
        }
    }

    fun clear() {
        data.clear()
    }

    fun size(): Int = data.size

    fun readPacket(): ByteArray? {
        if (data.size < 4) return null
        val len = (data[0].toInt() and 0xFF) or
            ((data[1].toInt() and 0xFF) shl 8) or
            ((data[2].toInt() and 0xFF) shl 16) or
            ((data[3].toInt() and 0xFF) shl 24)
        if (len <= 0) {
            data.subList(0, 4).clear()
            return null
        }
        if (data.size < 4 + len) return null
        val packet = ByteArray(len)
        for (i in 0 until len) {
            packet[i] = data[4 + i]
        }
        data.subList(0, 4 + len).clear()
        return packet
    }
}
