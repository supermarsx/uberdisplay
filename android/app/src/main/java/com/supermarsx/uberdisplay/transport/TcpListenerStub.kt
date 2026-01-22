package com.supermarsx.uberdisplay.transport

import java.net.ServerSocket
import java.net.Socket
import com.supermarsx.uberdisplay.media.CodecCapabilities
import com.supermarsx.uberdisplay.protocol.FramedPacketWriter
import com.supermarsx.uberdisplay.protocol.Packet

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
        TransportStatus.lastTcpConnectionAt = System.currentTimeMillis()
        try {
            TransportStatus.tcpState = TransportStatus.State.LISTENING
            socket.soTimeout = 5000
            val senderLoop = TcpSenderLoop(TransportOutbox.tcpQueue)
            senderLoop.start(socket.getOutputStream())
            sendCapabilities()
            TcpPacketLoop().run(socket.getInputStream())
            senderLoop.stop()
            socket.close()
        } catch (_: Exception) {
        }
    }

    private fun sendCapabilities() {
        val codecMask = CodecCapabilities.getCodecMask()
        val packet = Packet.Capabilities(codecMask = codecMask, flags = 0)
        val framed = FramedPacketWriter().write(packet)
        if (framed.isNotEmpty()) {
            TransportOutbox.tcpQueue.enqueue(framed)
        }
    }
}
