package com.supermarsx.uberdisplay.actionmenu

import com.supermarsx.uberdisplay.protocol.FramedPacketWriter
import com.supermarsx.uberdisplay.protocol.Packet
import com.supermarsx.uberdisplay.transport.TransportOutbox

class ActionMenuSender(
    private val writer: FramedPacketWriter = FramedPacketWriter()
) {
    fun sendTap(item: ActionMenuItem) {
        // Use InputKey down+up to represent a tap-style menu action.
        sendInputKey(item, true)
        sendInputKey(item, false)
    }

    fun sendConfig(item: ActionMenuItem) {
        // Use InputConfig to represent a menu button function.
        val packet = Packet.InputConfig(buttonFunction = item.actionId)
        val bytes = writer.write(packet)
        enqueue(bytes)
    }

    private fun sendInputKey(item: ActionMenuItem, down: Boolean) {
        val packet = Packet.InputKey(down = down, buttonIndex = item.id, actionId = item.actionId)
        val bytes = writer.write(packet)
        enqueue(bytes)
    }

    private fun enqueue(bytes: ByteArray) {
        if (bytes.isNotEmpty()) {
            TransportOutbox.tcpQueue.enqueue(bytes)
        }
    }
}
