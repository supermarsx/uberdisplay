package com.supermarsx.uberdisplay.media

import android.media.MediaCodecList
import com.supermarsx.uberdisplay.protocol.CodecConstants

object CodecCapabilities {
    fun getCodecMask(): Int {
        return try {
            val list = MediaCodecList(MediaCodecList.ALL_CODECS)
            var mask = 0
            for (info in list.codecInfos) {
                if (info.isEncoder) continue
                val types = info.supportedTypes.map { it.lowercase() }
                if (types.any { it.contains("video/avc") }) {
                    mask = mask or CodecConstants.CODEC_MASK_H264
                }
                if (types.any { it.contains("video/hevc") }) {
                    mask = mask or CodecConstants.CODEC_MASK_H265
                }
                if (types.any { it.contains("video/av01") }) {
                    mask = mask or CodecConstants.CODEC_MASK_AV1
                }
                if (types.any { it.contains("video/x-vnd.on2.vp9") }) {
                    mask = mask or CodecConstants.CODEC_MASK_VP9
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
