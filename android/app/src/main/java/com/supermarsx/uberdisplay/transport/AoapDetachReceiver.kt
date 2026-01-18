package com.supermarsx.uberdisplay.transport

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import com.supermarsx.uberdisplay.Diagnostics

class AoapDetachReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        Diagnostics.logInfo("aoap_detach_received action=${intent.action}")
        TransportStatus.aoapState = TransportStatus.State.WAITING
    }
}
