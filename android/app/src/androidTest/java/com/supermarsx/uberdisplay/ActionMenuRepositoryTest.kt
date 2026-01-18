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

    @Test
    fun updatesFirstItem() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val repo = ActionMenuRepository(context)
        val items = listOf(ActionMenuItem(1, "Test", 123))
        repo.saveItems(items)

        val updated = listOf(ActionMenuItem(1, "Test*", 124))
        repo.saveItems(updated)
        val loaded = repo.getItems()
        assertEquals("Test*", loaded[0].title)
        assertEquals(124, loaded[0].actionId)
    }

    @Test
    fun capsItemsAtTen() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val repo = ActionMenuRepository(context)
        val items = (0..15).map { ActionMenuItem(it, "Item $it", 1000 + it) }
        repo.saveItems(items)

        val loaded = repo.getItems()
        assertEquals(10, loaded.size)
    }
}
