package com.supermarsx.uberdisplay.ui

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.supermarsx.uberdisplay.R
import com.supermarsx.uberdisplay.actionmenu.ActionMenuRepository
import android.widget.TextView
import android.widget.Button
import com.supermarsx.uberdisplay.actionmenu.ActionMenuItem

class ActionMenuEditActivity : AppCompatActivity() {
    private lateinit var repo: ActionMenuRepository
    private lateinit var listView: TextView

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_action_menu_edit)

        repo = ActionMenuRepository(this)
        listView = findViewById(R.id.actionMenuEditList)
        val addButton = findViewById<Button>(R.id.actionMenuAddItem)

        addButton.setOnClickListener {
            val items = repo.getItems().toMutableList()
            val nextId = (items.maxOfOrNull { it.id } ?: 0) + 1
            items.add(ActionMenuItem(nextId, "Custom $nextId", 3000 + nextId))
            repo.saveItems(items)
            renderItems(items)
        }

        renderItems(repo.getItems())
    }

    private fun renderItems(items: List<ActionMenuItem>) {
        val formatted = items.joinToString("\n") { "- ${it.title} (${it.actionId})" }
        listView.text = if (formatted.isBlank()) {
            getString(R.string.action_menu_edit_list_placeholder)
        } else {
            formatted
        }
    }
}
