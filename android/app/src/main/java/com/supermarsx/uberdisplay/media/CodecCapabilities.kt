package com.supermarsx.uberdisplay.media

import android.media.MediaCodecList
import com.supermarsx.uberdisplay.protocol.CodecConstants

object CodecCapabilities {
    private val mimeToMask = mapOf(
        "video/avc" to CodecConstants.CODEC_MASK_H264,
        "video/hevc" to CodecConstants.CODEC_MASK_H265,
        "video/av01" to CodecConstants.CODEC_MASK_AV1,
        "video/x-vnd.on2.vp9" to CodecConstants.CODEC_MASK_VP9,
        "video/evc" to CodecConstants.CODEC_MASK_EVC,
        "video/lcevc" to CodecConstants.CODEC_MASK_LCEVC
    )

    fun getCodecMask(): Int {
        return try {
            val list = MediaCodecList(MediaCodecList.ALL_CODECS)
            var mask = 0
            for (info in list.codecInfos) {
                if (info.isEncoder) continue
                val types = info.supportedTypes.map { it.lowercase() }
                for ((mime, codecMask) in mimeToMask) {
                    if (types.any { it.contains(mime) }) {
                        mask = mask or codecMask
                    }
                }
            }
            if (mask == 0) {
                CodecConstants.CODEC_MASK_H264
            } else {
                mask
            }
        } catch (_: Throwable) {
            CodecConstants.CODEC_MASK_H264
        }
    }
}
