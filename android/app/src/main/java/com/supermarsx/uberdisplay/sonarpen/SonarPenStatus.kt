package com.supermarsx.uberdisplay.sonarpen

import android.Manifest
import android.content.Context
import android.content.pm.PackageManager
import androidx.core.content.ContextCompat

object SonarPenStatus {
    enum class Status {
        UNKNOWN,
        NOT_ENABLED,
        READY
    }

    fun currentStatus(context: Context): Status {
        val granted = ContextCompat.checkSelfPermission(
            context,
            Manifest.permission.RECORD_AUDIO
        ) == PackageManager.PERMISSION_GRANTED

        return if (granted) {
            Status.READY
        } else {
            Status.NOT_ENABLED
        }
    }
}
