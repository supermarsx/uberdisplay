package com.supermarsx.uberdisplay

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import com.google.android.material.switchmaterial.SwitchMaterial
import android.content.Intent
import android.widget.Button
import com.supermarsx.uberdisplay.settings.SettingsActivity
import androidx.preference.PreferenceManager
import com.supermarsx.uberdisplay.ui.MirrorActivity
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.lifecycleScope
import androidx.lifecycle.repeatOnLifecycle
import com.google.android.material.chip.Chip
import kotlinx.coroutines.launch

class MainActivity : AppCompatActivity() {
    private lateinit var rootToggle: SwitchMaterial
    private lateinit var rootStatus: TextView
    private lateinit var statusChip: Chip
    private lateinit var settingsButton: Button
    private lateinit var connectButton: Button
    private val connectionController = AppServices.connectionController
    private var lastState: ConnectionState = ConnectionState.IDLE

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        rootToggle = findViewById(R.id.rootToggle)
        rootStatus = findViewById(R.id.rootStatus)
        statusChip = findViewById(R.id.statusChip)
        settingsButton = findViewById(R.id.settingsButton)
        connectButton = findViewById(R.id.connectButton)
        AppServices.init(this)
        val prefs = PreferenceManager.getDefaultSharedPreferences(this)

        rootToggle.isChecked = prefs.getBoolean("use_root_module", false)
        updateRootStatus()
        bindConnectionState()

        rootToggle.setOnCheckedChangeListener { _, isChecked ->
            prefs.edit().putBoolean("use_root_module", isChecked).apply()
            updateRootStatus()
        }

        settingsButton.setOnClickListener {
            startActivity(Intent(this, SettingsActivity::class.java))
        }

        connectButton.setOnClickListener {
            if (lastState == ConnectionState.IDLE || lastState == ConnectionState.ERROR) {
                connectionController.startTcp()
                connectionController.markConnected()
                startActivity(Intent(this, MirrorActivity::class.java))
            } else {
                connectionController.stop()
            }
        }
    }

    private fun bindConnectionState() {
        lifecycleScope.launch {
            repeatOnLifecycle(Lifecycle.State.STARTED) {
                connectionController.stateStore().state.collect { state ->
                    updateConnectionState(state)
                }
            }
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
        statusChip.setText(textRes)
        val background = when (state) {
            ConnectionState.IDLE -> R.color.status_idle_bg
            ConnectionState.WAITING -> R.color.status_waiting_bg
            ConnectionState.CONNECTED -> R.color.status_connected_bg
            ConnectionState.ERROR -> R.color.status_error_bg
        }
        statusChip.setChipBackgroundColorResource(background)

        lastState = state
        connectButton.setText(
            if (state == ConnectionState.CONNECTED || state == ConnectionState.WAITING) {
                R.string.disconnect
            } else {
                R.string.connect
            }
        )
    }
}
}
