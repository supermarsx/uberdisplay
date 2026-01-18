package com.supermarsx.uberdisplay.actionmenu

class ActionMenuRepository {
    fun getItems(): List<ActionMenuItem> {
        return defaultItems()
    }

    private fun defaultItems(): List<ActionMenuItem> {
        return listOf(
            ActionMenuItem(0, "Esc", 1001),
            ActionMenuItem(1, "Enter", 1002),
            ActionMenuItem(2, "Tab", 1003),
            ActionMenuItem(3, "Screenshot", 2001)
        )
    }
}
