package com.supermarsx.uberdisplay.ui

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.supermarsx.uberdisplay.R
import com.supermarsx.uberdisplay.AppServices
import com.supermarsx.uberdisplay.input.InputSenderStub
import com.supermarsx.uberdisplay.input.PenInputHandler
import com.supermarsx.uberdisplay.input.TouchInputHandler
import android.view.KeyEvent
import android.widget.Button
import android.widget.Toast
import android.content.Intent
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.lifecycleScope
import androidx.lifecycle.repeatOnLifecycle
import kotlinx.coroutines.launch
import androidx.preference.PreferenceManager
import android.widget.LinearLayout
import com.supermarsx.uberdisplay.actionmenu.ActionMenuRepository
import com.supermarsx.uberdisplay.actionmenu.ActionMenuSender

class MirrorActivity : AppCompatActivity() {
    private val inputSender = InputSenderStub()
    private var lastState: com.supermarsx.uberdisplay.ConnectionState =
        com.supermarsx.uberdisplay.ConnectionState.IDLE
    private val actionMenuSender = ActionMenuSender()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_mirror)

        savedInstanceState?.getString(KEY_LAST_STATE)?.let { name ->
            lastState = runCatching {
                com.supermarsx.uberdisplay.ConnectionState.valueOf(name)
            }.getOrDefault(lastState)
        }

        val root = findViewById<android.view.View>(R.id.mirrorRoot)
        root.isFocusableInTouchMode = true
        root.requestFocus()

        root.setOnTouchListener(TouchInputHandler(inputSender))
        root.setOnGenericMotionListener(PenInputHandler(inputSender))

        val actionButton = findViewById<Button>(R.id.actionMenuButton)
        actionButton.setOnClickListener {
            startActivity(Intent(this, ActionMenuActivity::class.java))
            Toast.makeText(this, R.string.action_menu_placeholder, Toast.LENGTH_SHORT).show()
        }

        renderActionMenu()

        val disconnectButton = findViewById<Button>(R.id.disconnectButton)
        disconnectButton.setOnClickListener {
            AppServices.connectionController.stop()
            finish()
        }

        val toggleButton = findViewById<Button>(R.id.sessionToggleButton)
        toggleButton.setOnClickListener {
            val controller = AppServices.connectionController
            if (lastState == com.supermarsx.uberdisplay.ConnectionState.CONNECTED ||
                lastState == com.supermarsx.uberdisplay.ConnectionState.WAITING
            ) {
                controller.stop()
            } else {
                val prefs = PreferenceManager.getDefaultSharedPreferences(this)
                when (prefs.getString("connection_mode", "tcp")) {
                    "aoap" -> controller.startAoap()
                    else -> controller.startTcp()
                }
                controller.markConnected()
            }
        }
    }

    override fun onStart() {
        super.onStart()
        AppServices.connectionController.markConnected()
        bindSessionStatus()
    }

    override fun onStop() {
        super.onStop()
        AppServices.connectionController.stop()
    }

    override fun onResume() {
        super.onResume()
        renderActionMenu()
    }

    private fun bindSessionStatus() {
        val statusView = findViewById<android.widget.TextView>(R.id.sessionStatus)
        val toggleButton = findViewById<Button>(R.id.sessionToggleButton)
        lifecycleScope.launch {
            repeatOnLifecycle(Lifecycle.State.STARTED) {
                AppServices.connectionController.stateStore().state.collect { state ->
                    val textRes = when (state) {
                        com.supermarsx.uberdisplay.ConnectionState.IDLE ->
                            R.string.session_status_idle
                        com.supermarsx.uberdisplay.ConnectionState.WAITING ->
                            R.string.session_status_waiting
                        com.supermarsx.uberdisplay.ConnectionState.CONNECTED ->
                            R.string.session_status_connected
                        com.supermarsx.uberdisplay.ConnectionState.ERROR ->
                            R.string.session_status_error
                    }
                    statusView.setText(textRes)
                    lastState = state
                    toggleButton.setText(
                        if (state == com.supermarsx.uberdisplay.ConnectionState.CONNECTED ||
                            state == com.supermarsx.uberdisplay.ConnectionState.WAITING
                        ) {
                            R.string.session_pause
                        } else {
                            R.string.session_resume
                        }
                    )
                }
            }
        }
    }

    private fun renderActionMenu() {
        val container = findViewById<LinearLayout>(R.id.actionMenuContainer)
        container.removeAllViews()
        val repo = ActionMenuRepository(this)
        val items = repo.getItems().take(10)
        for (item in items) {
            val button = Button(this)
            button.text = item.title
            button.setOnClickListener {
                actionMenuSender.sendTap(item)
            }
            container.addView(button)
        }
        if (items.isEmpty()) {
            val hint = android.widget.TextView(this)
            hint.text = getString(R.string.action_menu_empty_hint)
            container.addView(hint)
        }
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent?): Boolean {
        inputSender.sendKey(keyCode, true)
        return super.onKeyDown(keyCode, event)
    }

    override fun onKeyUp(keyCode: Int, event: KeyEvent?): Boolean {
        inputSender.sendKey(keyCode, false)
        return super.onKeyUp(keyCode, event)
    }

    override fun onSaveInstanceState(outState: Bundle) {
        outState.putString(KEY_LAST_STATE, lastState.name)
        super.onSaveInstanceState(outState)
    }

    companion object {
        private const val KEY_LAST_STATE = "mirror_last_state"
    }
}
