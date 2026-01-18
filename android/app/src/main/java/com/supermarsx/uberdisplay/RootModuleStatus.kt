package com.supermarsx.uberdisplay

import android.net.LocalSocket
import android.net.LocalSocketAddress
import java.io.BufferedReader
import java.io.File
import java.io.InputStreamReader

object RootModuleStatus {
    private const val ROOT_SOCKET_PATH = "/data/local/tmp/uberdisplay/root.sock"

    enum class Status {
        NOT_DETECTED,
        UNREACHABLE,
        HANDSHAKE_OK
    }

    data class Handshake(val ok: Boolean, val caps: Long)

    fun isSocketPresent(): Boolean {
        return File(ROOT_SOCKET_PATH).exists()
    }

    fun checkStatus(): Status {
        if (!File(ROOT_SOCKET_PATH).exists()) {
            return Status.NOT_DETECTED
        }

        return try {
            val socket = LocalSocket()
            socket.soTimeout = 500
            socket.connect(LocalSocketAddress(ROOT_SOCKET_PATH, LocalSocketAddress.Namespace.FILESYSTEM))
            val output = socket.outputStream
            val input = BufferedReader(InputStreamReader(socket.inputStream))

            output.write("HELLO 1\n".toByteArray())
            output.write("PING\n".toByteArray())
            output.flush()

            val hello = input.readLine() ?: ""
            val pong = input.readLine() ?: ""
            socket.close()

            val handshake = parseHandshake(hello, pong)
            if (handshake.ok) {
                Status.HANDSHAKE_OK
            } else {
                Status.UNREACHABLE
            }
        } catch (_: Exception) {
            Status.UNREACHABLE
        }
    }

    fun parseHandshake(helloLine: String, pongLine: String): Handshake {
        if (!helloLine.startsWith("OK") || !pongLine.startsWith("PONG")) {
            return Handshake(false, 0)
        }

        val capsToken = helloLine.split(" ").firstOrNull { it.startsWith("caps=") }
        val capsValue = capsToken?.substringAfter("caps=")?.removePrefix("0x")
        val caps = capsValue?.toLongOrNull(16) ?: 0
        return Handshake(true, caps)
    }

    fun checkHandshakeCaps(): Handshake {
        if (!File(ROOT_SOCKET_PATH).exists()) {
            return Handshake(false, 0)
        }

        return try {
            val socket = LocalSocket()
            socket.soTimeout = 500
            socket.connect(LocalSocketAddress(ROOT_SOCKET_PATH, LocalSocketAddress.Namespace.FILESYSTEM))
            val output = socket.outputStream
            val input = BufferedReader(InputStreamReader(socket.inputStream))

            output.write("HELLO 1\n".toByteArray())
            output.write("PING\n".toByteArray())
            output.flush()

            val hello = input.readLine() ?: ""
            val pong = input.readLine() ?: ""
            socket.close()

            parseHandshake(hello, pong)
        } catch (_: Exception) {
            Handshake(false, 0)
        }
    }
}
