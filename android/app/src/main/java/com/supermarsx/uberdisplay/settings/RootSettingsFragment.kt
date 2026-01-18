package com.supermarsx.uberdisplay.settings

import android.os.Bundle
import androidx.preference.Preference
import androidx.preference.PreferenceFragmentCompat
import androidx.preference.SwitchPreferenceCompat
import com.supermarsx.uberdisplay.R
import com.supermarsx.uberdisplay.RootModuleStatus
import com.supermarsx.uberdisplay.Diagnostics
import com.supermarsx.uberdisplay.sonarpen.SonarPenCalibrationActivity
import com.supermarsx.uberdisplay.sonarpen.SonarPenStatus
import android.content.Intent
import com.supermarsx.uberdisplay.transport.TransportStatus
import com.supermarsx.uberdisplay.protocol.ProtocolConstants

class RootSettingsFragment : PreferenceFragmentCompat() {
    override fun onCreatePreferences(savedInstanceState: Bundle?, rootKey: String?) {
        setPreferencesFromResource(R.xml.preferences, rootKey)
        wireSonarPenPreferences()
        wireDiagnosticsPreferences()
        updateTransportStatus()
    }

    override fun onResume() {
        super.onResume()
        updateRootStatus()
        updateSonarPenStatus()
        updateTransportStatus()
    }

    private fun updateRootStatus() {
        val statusPref = findPreference<Preference>("root_module_status") ?: return
        statusPref.summary = getString(R.string.root_module_status_checking)

        Thread {
            val handshake = RootModuleStatus.checkHandshakeCaps()
            val socketPresent = RootModuleStatus.isSocketPresent()
            activity?.runOnUiThread {
                statusPref.summary = when {
                    handshake.ok -> {
                        val capsHex = "0x" + handshake.caps.toString(16).padStart(8, '0')
                        getString(R.string.root_module_status_detected_with_caps, capsHex)
                    }
                    !socketPresent -> getString(R.string.root_module_status_not_detected)
                    else -> getString(R.string.root_module_status_unreachable_detail)
                }
            }
        }.start()
    }

    private fun wireSonarPenPreferences() {
        val statusPref = findPreference<Preference>("sonarpen_status")
        val calibratePref = findPreference<Preference>("sonarpen_calibrate")

        calibratePref?.setOnPreferenceClickListener {
            startActivity(Intent(requireContext(), SonarPenCalibrationActivity::class.java))
            true
        }
    }

    private fun updateSonarPenStatus() {
        val statusPref = findPreference<Preference>("sonarpen_status") ?: return
        statusPref.summary = when (SonarPenStatus.currentStatus(requireContext())) {
            SonarPenStatus.Status.UNKNOWN -> getString(R.string.sonarpen_status_unknown)
            SonarPenStatus.Status.NOT_ENABLED -> getString(R.string.sonarpen_permission_needed)
            SonarPenStatus.Status.READY -> getString(R.string.sonarpen_permission_granted)
        }
    }

    private fun wireDiagnosticsPreferences() {
        val diagnosticsPref = findPreference<SwitchPreferenceCompat>("diagnostics_enabled")
        diagnosticsPref?.setOnPreferenceChangeListener { _, newValue ->
            Diagnostics.setEnabled(newValue as Boolean)
            true
        }
    }

    private fun updateTransportStatus() {
        val tcpPortPref = findPreference<Preference>("tcp_port")
        val tcpOutboxPref = findPreference<Preference>("tcp_outbox")
        val aoapPref = findPreference<Preference>("aoap_status")
        val tcpStateLabel = when (TransportStatus.tcpState) {
            TransportStatus.State.STOPPED -> getString(R.string.transport_state_stopped)
            TransportStatus.State.LISTENING -> getString(R.string.transport_state_listening)
            TransportStatus.State.WAITING -> getString(R.string.transport_state_waiting)
        }
        val lastSeen = formatLastConnection(TransportStatus.lastTcpConnectionAt)
        tcpPortPref?.summary = getString(
            R.string.tcp_port_summary_with_state,
            ProtocolConstants.DEFAULT_TCP_PORT,
            tcpStateLabel,
            TransportStatus.tcpConnections,
            lastSeen
        )
        val outboxSize = com.supermarsx.uberdisplay.transport.TransportOutbox.tcpQueue.size()
        tcpOutboxPref?.summary = getString(R.string.tcp_outbox_summary, outboxSize)
        aoapPref?.summary = when (TransportStatus.aoapState) {
            TransportStatus.State.STOPPED -> getString(R.string.aoap_status_stopped)
            TransportStatus.State.WAITING -> getString(R.string.aoap_status_waiting)
            TransportStatus.State.LISTENING -> getString(R.string.aoap_status_listening)
        }
    }

    private fun formatLastConnection(timestamp: Long): String {
        if (timestamp <= 0) return getString(R.string.tcp_last_connection_never)
        val seconds = (System.currentTimeMillis() - timestamp) / 1000
        return getString(R.string.tcp_last_connection_seconds, seconds)
    }
}
