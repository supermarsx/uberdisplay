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

class RootSettingsFragment : PreferenceFragmentCompat() {
    override fun onCreatePreferences(savedInstanceState: Bundle?, rootKey: String?) {
        setPreferencesFromResource(R.xml.preferences, rootKey)
        wireSonarPenPreferences()
        wireDiagnosticsPreferences()
    }

    override fun onResume() {
        super.onResume()
        updateRootStatus()
        updateSonarPenStatus()
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
}
