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

            if (hello.startsWith("OK") && pong.startsWith("PONG")) {
                Status.HANDSHAKE_OK
            } else {
                Status.UNREACHABLE
            }
        } catch (_: Exception) {
            Status.UNREACHABLE
        }
    }
}
