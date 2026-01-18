package com.supermarsx.uberdisplay.actionmenu

import android.content.Context
import androidx.preference.PreferenceManager

class ActionMenuRepository(private val context: Context) {
    fun getItems(): List<ActionMenuItem> {
        val prefs = PreferenceManager.getDefaultSharedPreferences(context)
        val raw = prefs.getString(KEY_ITEMS, null) ?: return defaultItems()
        val items = parseItems(raw)
        return if (items.isEmpty()) defaultItems() else items
    }

    fun saveItems(items: List<ActionMenuItem>) {
        val prefs = PreferenceManager.getDefaultSharedPreferences(context)
        prefs.edit().putString(KEY_ITEMS, serializeItems(items)).apply()
    }

    private fun defaultItems(): List<ActionMenuItem> {
        return listOf(
            ActionMenuItem(0, "Esc", 1001),
            ActionMenuItem(1, "Enter", 1002),
            ActionMenuItem(2, "Tab", 1003),
            ActionMenuItem(3, "Screenshot", 2001)
        )
    }

    private fun serializeItems(items: List<ActionMenuItem>): String {
        return items.joinToString("|") { "${it.id}:${it.title}:${it.actionId}" }
    }

    private fun parseItems(value: String): List<ActionMenuItem> {
        return value.split("|").mapNotNull { token ->
            val parts = token.split(":")
            if (parts.size != 3) return@mapNotNull null
            val id = parts[0].toIntOrNull() ?: return@mapNotNull null
            val title = parts[1]
            val actionId = parts[2].toIntOrNull() ?: return@mapNotNull null
            ActionMenuItem(id, title, actionId)
        }
    }

    companion object {
        private const val KEY_ITEMS = "action_menu_items"
    }
}
