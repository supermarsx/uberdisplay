package com.supermarsx.uberdisplay.protocol

object CodecConstants {
    const val CODEC_ID_H264 = 1
    const val CODEC_ID_H265 = 2
    const val CODEC_ID_AV1 = 3
    const val CODEC_ID_VP9 = 4
    const val CODEC_ID_H266 = 5

    const val CODEC_MASK_H264 = 1 shl 0
    const val CODEC_MASK_H265 = 1 shl 1
    const val CODEC_MASK_AV1 = 1 shl 2
    const val CODEC_MASK_VP9 = 1 shl 3
    const val CODEC_MASK_H266 = 1 shl 4
}
