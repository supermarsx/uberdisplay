package com.supermarsx.uberdisplay.media

import android.view.Surface
import com.supermarsx.uberdisplay.Diagnostics
import com.supermarsx.uberdisplay.protocol.CodecConstants
import com.supermarsx.uberdisplay.protocol.Packet

data class DecoderStatus(
    val codecId: Int,
    val mimeType: String,
    val width: Int,
    val height: Int,
    val surfaceBound: Boolean
)

class DecoderController {
    @Volatile
    private var surface: Surface? = null
    @Volatile
    private var status = DecoderStatus(
        codecId = CodecConstants.CODEC_ID_H264,
        mimeType = "video/avc",
        width = 0,
        height = 0,
        surfaceBound = false
    )

    fun setSurface(surface: Surface?) {
        this.surface = surface
        status = status.copy(surfaceBound = surface != null)
    }

    fun onConfigure(packet: Packet.Configure) {
        val codecId = packet.codecId ?: CodecConstants.CODEC_ID_H264
        val mime = codecIdToMime(codecId)
        status = status.copy(
            codecId = codecId,
            mimeType = mime,
            width = packet.width,
            height = packet.height
        )
        Diagnostics.logInfo("decoder_config codec=$codecId mime=$mime ${packet.width}x${packet.height}")
    }

    fun onFrame(data: ByteArray) {
        if (data.isEmpty()) return
        // TODO: feed MediaCodec once decoder pipeline is wired.
    }

    fun getStatus(): DecoderStatus = status

    private fun codecIdToMime(codecId: Int): String {
        return when (codecId) {
            CodecConstants.CODEC_ID_H265 -> "video/hevc"
            CodecConstants.CODEC_ID_AV1 -> "video/av01"
            CodecConstants.CODEC_ID_VP9 -> "video/x-vnd.on2.vp9"
            CodecConstants.CODEC_ID_EVC -> "video/evc"
            CodecConstants.CODEC_ID_LCEVC -> "video/lcevc"
            CodecConstants.CODEC_ID_H266 -> "video/avc"
            else -> "video/avc"
        }
    }
}
