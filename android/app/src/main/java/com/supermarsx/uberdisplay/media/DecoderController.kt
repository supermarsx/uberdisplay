package com.supermarsx.uberdisplay.media

import android.media.MediaCodec
import android.media.MediaFormat
import android.view.Surface
import com.supermarsx.uberdisplay.Diagnostics
import com.supermarsx.uberdisplay.protocol.CodecConstants
import com.supermarsx.uberdisplay.protocol.Packet

data class DecoderStatus(
    val codecId: Int,
    val mimeType: String,
    val width: Int,
    val height: Int,
    val surfaceBound: Boolean,
    val decoderStarted: Boolean = false
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

    private var decoder: MediaCodec? = null
    private val lock = Any()
    private var presentationTimeUs = 0L

    fun setSurface(surface: Surface?) {
        synchronized(lock) {
            this.surface = surface
            status = status.copy(surfaceBound = surface != null)
            if (surface != null && status.width > 0 && status.height > 0) {
                initDecoder()
            } else if (surface == null) {
                releaseDecoder()
            }
        }
    }

    fun onConfigure(packet: Packet.Configure) {
        synchronized(lock) {
            val codecId = packet.codecId ?: CodecConstants.CODEC_ID_H264
            val mime = codecIdToMime(codecId)
            val changed = codecId != status.codecId ||
                packet.width != status.width ||
                packet.height != status.height
            status = status.copy(
                codecId = codecId,
                mimeType = mime,
                width = packet.width,
                height = packet.height
            )
            Diagnostics.logInfo("decoder_config codec=$codecId mime=$mime ${packet.width}x${packet.height}")
            if (changed && surface != null && packet.width > 0 && packet.height > 0) {
                initDecoder()
            }
        }
    }

    fun onFrame(data: ByteArray) {
        if (data.isEmpty()) return
        synchronized(lock) {
            feedDecoder(data)
        }
    }

    fun release() {
        synchronized(lock) {
            releaseDecoder()
        }
    }

    fun getStatus(): DecoderStatus = status

    private fun initDecoder() {
        releaseDecoder()
        val currentSurface = surface ?: return
        val mime = status.mimeType
        val format = MediaFormat.createVideoFormat(mime, status.width, status.height)
        try {
            val codec = MediaCodec.createDecoderByType(mime)
            codec.configure(format, currentSurface, null, 0)
            codec.start()
            decoder = codec
            presentationTimeUs = 0L
            status = status.copy(decoderStarted = true)
            Diagnostics.logInfo("decoder_init mime=$mime ${status.width}x${status.height}")
        } catch (e: Exception) {
            Diagnostics.logError("decoder_init_failed ${e.message}", e)
            status = status.copy(decoderStarted = false)
        }
    }

    private fun feedDecoder(data: ByteArray) {
        val codec = decoder ?: return
        try {
            val inputIndex = codec.dequeueInputBuffer(INPUT_TIMEOUT_US)
            if (inputIndex >= 0) {
                val inputBuffer = codec.getInputBuffer(inputIndex) ?: return
                inputBuffer.clear()
                inputBuffer.put(data, 0, data.size.coerceAtMost(inputBuffer.capacity()))
                codec.queueInputBuffer(inputIndex, 0, data.size.coerceAtMost(inputBuffer.capacity()), presentationTimeUs, 0)
                presentationTimeUs += FRAME_DURATION_US
            }
            drainOutput(codec)
        } catch (e: MediaCodec.CodecException) {
            Diagnostics.logError("decoder_feed_error ${e.diagnosticInfo}", e)
            if (!e.isRecoverable) {
                initDecoder()
            }
        } catch (e: IllegalStateException) {
            Diagnostics.logError("decoder_feed_illegal_state ${e.message}", e)
        }
    }

    private fun drainOutput(codec: MediaCodec) {
        val info = MediaCodec.BufferInfo()
        while (true) {
            val outputIndex = codec.dequeueOutputBuffer(info, OUTPUT_TIMEOUT_US)
            when {
                outputIndex >= 0 -> {
                    codec.releaseOutputBuffer(outputIndex, info.size > 0)
                    if (info.flags and MediaCodec.BUFFER_FLAG_END_OF_STREAM != 0) {
                        return
                    }
                }
                outputIndex == MediaCodec.INFO_OUTPUT_FORMAT_CHANGED -> {
                    val newFormat = codec.outputFormat
                    Diagnostics.logInfo("decoder_format_changed $newFormat")
                }
                else -> return
            }
        }
    }

    private fun releaseDecoder() {
        try {
            decoder?.stop()
        } catch (_: Exception) {}
        try {
            decoder?.release()
        } catch (_: Exception) {}
        decoder = null
        status = status.copy(decoderStarted = false)
    }

    companion object {
        private const val INPUT_TIMEOUT_US = 5_000L
        private const val OUTPUT_TIMEOUT_US = 0L
        private const val FRAME_DURATION_US = 16_667L

        fun codecIdToMime(codecId: Int): String {
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
}
