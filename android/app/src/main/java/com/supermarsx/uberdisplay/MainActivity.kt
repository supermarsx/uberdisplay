package com.supermarsx.uberdisplay

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import com.google.android.material.switchmaterial.SwitchMaterial

class MainActivity : AppCompatActivity() {
    private lateinit var rootToggle: SwitchMaterial
    private lateinit var rootStatus: TextView
    private lateinit var statusValue: TextView

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        rootToggle = findViewById(R.id.rootToggle)
        rootStatus = findViewById(R.id.rootStatus)
        statusValue = findViewById(R.id.statusValue)
        val prefs = getSharedPreferences("uberdisplay_prefs", MODE_PRIVATE)

        rootToggle.isChecked = prefs.getBoolean("use_root_module", false)
        updateRootStatus()
        updateConnectionState(ConnectionState.IDLE)

        rootToggle.setOnCheckedChangeListener { _, isChecked ->
            prefs.edit().putBoolean("use_root_module", isChecked).apply()
            updateRootStatus()
        }
    }

    private fun updateRootStatus() {
        if (!rootToggle.isChecked) {
            rootStatus.setText(R.string.root_module_status_disabled)
            return
        }

        rootStatus.setText(R.string.root_module_status_checking)
        Thread {
            val status = RootModuleStatus.checkStatus()
            runOnUiThread {
                if (!rootToggle.isChecked) {
                    rootStatus.setText(R.string.root_module_status_disabled)
                    return@runOnUiThread
                }
                when (status) {
                    RootModuleStatus.Status.HANDSHAKE_OK ->
                        rootStatus.setText(R.string.root_module_status_detected)
                    RootModuleStatus.Status.NOT_DETECTED ->
                        rootStatus.setText(R.string.root_module_status_not_detected)
                    RootModuleStatus.Status.UNREACHABLE ->
                        rootStatus.setText(R.string.root_module_status_unreachable)
                }
            }
        }.start()
    }

    private fun updateConnectionState(state: ConnectionState) {
        val textRes = when (state) {
            ConnectionState.IDLE -> R.string.status_idle
            ConnectionState.WAITING -> R.string.status_waiting
            ConnectionState.CONNECTED -> R.string.status_connected
            ConnectionState.ERROR -> R.string.status_error
        }
        statusValue.setText(textRes)
    }
}
}
