package com.supermarsx.uberdisplay.ui

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.supermarsx.uberdisplay.R
import com.supermarsx.uberdisplay.actionmenu.ActionMenuRepository
import android.widget.TextView
import android.widget.Button
import android.content.Intent
import android.widget.Toast
import com.supermarsx.uberdisplay.actionmenu.ActionMenuSender

class ActionMenuActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_action_menu)

        val repo = ActionMenuRepository(this)
        val sender = ActionMenuSender()
        val countView = findViewById<TextView>(R.id.actionMenuCount)
        countView.text = getString(R.string.action_menu_items_count, repo.getItems().size)

        val editButton = findViewById<Button>(R.id.actionMenuEdit)
        editButton.setOnClickListener {
            startActivity(Intent(this, ActionMenuEditActivity::class.java))
        }

        val sendButton = findViewById<Button>(R.id.actionMenuSendSample)
        sendButton.setOnClickListener {
            val first = repo.getItems().firstOrNull()
            if (first == null) {
                Toast.makeText(this, R.string.action_menu_empty, Toast.LENGTH_SHORT).show()
                return@setOnClickListener
            }
            sender.sendTap(first)
            Toast.makeText(this, getString(R.string.action_menu_sent, first.title), Toast.LENGTH_SHORT).show()
        }

        val configButton = findViewById<Button>(R.id.actionMenuSendConfig)
        configButton.setOnClickListener {
            val first = repo.getItems().firstOrNull()
            if (first == null) {
                Toast.makeText(this, R.string.action_menu_empty, Toast.LENGTH_SHORT).show()
                return@setOnClickListener
            }
            sender.sendConfig(first)
            Toast.makeText(this, getString(R.string.action_menu_config_sent, first.title), Toast.LENGTH_SHORT).show()
        }
    }
}
