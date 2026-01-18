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

class MirrorActivity : AppCompatActivity() {
    private val inputSender = InputSenderStub()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_mirror)

        val root = findViewById<android.view.View>(R.id.mirrorRoot)
        root.isFocusableInTouchMode = true
        root.requestFocus()

        root.setOnTouchListener(TouchInputHandler(inputSender))
        root.setOnGenericMotionListener(PenInputHandler(inputSender))

        val actionButton = findViewById<Button>(R.id.actionMenuButton)
        actionButton.setOnClickListener {
            Toast.makeText(this, R.string.action_menu_placeholder, Toast.LENGTH_SHORT).show()
        }
    }

    override fun onStart() {
        super.onStart()
        AppServices.connectionController.markConnected()
    }

    override fun onStop() {
        super.onStop()
        AppServices.connectionController.stop()
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent?): Boolean {
        inputSender.sendKey(keyCode, true)
        return super.onKeyDown(keyCode, event)
    }

    override fun onKeyUp(keyCode: Int, event: KeyEvent?): Boolean {
        inputSender.sendKey(keyCode, false)
        return super.onKeyUp(keyCode, event)
    }
}
