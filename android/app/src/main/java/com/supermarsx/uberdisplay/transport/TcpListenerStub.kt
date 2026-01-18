package com.supermarsx.uberdisplay.transport

import java.net.ServerSocket
import java.net.Socket

class TcpListenerStub(
    private val port: Int = TcpTransportStub.DEFAULT_PORT
) {
    @Volatile
    private var running = false
    @Volatile
    private var serverSocket: ServerSocket? = null
    @Volatile
    private var thread: Thread? = null

    fun start() {
        if (running) return
        running = true
        thread = Thread { runServer() }.also { it.start() }
    }

    fun stop() {
        running = false
        serverSocket?.close()
        thread?.interrupt()
        serverSocket = null
        thread = null
        TransportStatus.tcpState = TransportStatus.State.STOPPED
    }

    fun isRunning(): Boolean = running

    fun getPort(): Int = port

    private fun runServer() {
        try {
            ServerSocket(port).use { server ->
                serverSocket = server
                TransportStatus.tcpState = TransportStatus.State.LISTENING
                while (running) {
                    val socket = server.accept()
                    handleConnection(socket)
                }
            }
        } catch (_: Exception) {
            TransportStatus.tcpState = TransportStatus.State.STOPPED
        } finally {
            running = false
        }
    }

    private fun handleConnection(socket: Socket) {
        TransportStatus.tcpConnections += 1
        try {
            TransportStatus.tcpState = TransportStatus.State.LISTENING
            socket.soTimeout = 5000
            TcpPacketLoop().run(socket.getInputStream())
            socket.close()
        } catch (_: Exception) {
        }
    }
}
