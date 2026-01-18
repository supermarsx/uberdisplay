package com.supermarsx.uberdisplay.protocol

object Handshake {
    fun parseVersion(line: String): Int? {
        if (!line.startsWith(ProtocolConstants.HANDSHAKE_BASE)) return null
        val versionPart = line.substring(
            ProtocolConstants.HANDSHAKE_BASE.length,
            ProtocolConstants.HANDSHAKE_BASE.length + ProtocolConstants.HANDSHAKE_VERSION_LENGTH
        )
        return versionPart.toIntOrNull()
    }

    fun buildHello(version: Int): String {
        return ProtocolConstants.HANDSHAKE_BASE +
            version.toString().padStart(ProtocolConstants.HANDSHAKE_VERSION_LENGTH, '0') +
            "\u0000"
    }
}
