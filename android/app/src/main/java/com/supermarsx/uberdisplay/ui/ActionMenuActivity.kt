package com.supermarsx.uberdisplay.ui

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.supermarsx.uberdisplay.R
import com.supermarsx.uberdisplay.actionmenu.ActionMenuRepository
import android.widget.TextView
import android.widget.Button
import android.content.Intent

class ActionMenuActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_action_menu)

        val repo = ActionMenuRepository(this)
        val countView = findViewById<TextView>(R.id.actionMenuCount)
        countView.text = getString(R.string.action_menu_items_count, repo.getItems().size)

        val editButton = findViewById<Button>(R.id.actionMenuEdit)
        editButton.setOnClickListener {
            startActivity(Intent(this, ActionMenuEditActivity::class.java))
        }
    }
}
