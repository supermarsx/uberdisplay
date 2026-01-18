package com.supermarsx.uberdisplay.sonarpen

import android.Manifest
import android.content.pm.PackageManager
import android.os.Bundle
import android.widget.Button
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import com.supermarsx.uberdisplay.R

class SonarPenCalibrationActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_sonarpen_calibration)

        val permissionStatus = findViewById<TextView>(R.id.sonarpenPermissionStatus)
        val requestButton = findViewById<Button>(R.id.sonarpenRequestPermission)

        updatePermissionStatus(permissionStatus)

        requestButton.setOnClickListener {
            ActivityCompat.requestPermissions(
                this,
                arrayOf(Manifest.permission.RECORD_AUDIO),
                REQUEST_AUDIO
            )
        }
    }

    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        if (requestCode == REQUEST_AUDIO) {
            val permissionStatus = findViewById<TextView>(R.id.sonarpenPermissionStatus)
            updatePermissionStatus(permissionStatus)
        }
    }

    private fun updatePermissionStatus(view: TextView) {
        val granted = ContextCompat.checkSelfPermission(
            this,
            Manifest.permission.RECORD_AUDIO
        ) == PackageManager.PERMISSION_GRANTED
        view.setText(if (granted) R.string.sonarpen_permission_granted else R.string.sonarpen_permission_needed)
    }

    companion object {
        private const val REQUEST_AUDIO = 1001
    }
}
