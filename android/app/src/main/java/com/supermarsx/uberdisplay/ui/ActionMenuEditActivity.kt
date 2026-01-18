package com.supermarsx.uberdisplay.ui

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.supermarsx.uberdisplay.R
import com.supermarsx.uberdisplay.actionmenu.ActionMenuRepository
import android.widget.TextView

class ActionMenuEditActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_action_menu_edit)

        val repo = ActionMenuRepository(this)
        val listView = findViewById<TextView>(R.id.actionMenuEditList)
        val items = repo.getItems().joinToString("\n") { "- ${it.title} (${it.actionId})" }
        listView.text = items
    }
}
