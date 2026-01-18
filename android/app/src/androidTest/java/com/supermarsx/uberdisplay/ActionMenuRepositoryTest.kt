package com.supermarsx.uberdisplay

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import com.supermarsx.uberdisplay.actionmenu.ActionMenuItem
import com.supermarsx.uberdisplay.actionmenu.ActionMenuRepository
import org.junit.Assert.assertEquals
import org.junit.Test

class ActionMenuRepositoryTest {
    @Test
    fun savesAndLoadsItems() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val repo = ActionMenuRepository(context)
        val items = listOf(ActionMenuItem(1, "Test", 123))
        repo.saveItems(items)

        val loaded = repo.getItems()
        assertEquals(1, loaded.size)
        assertEquals("Test", loaded[0].title)
    }
}
