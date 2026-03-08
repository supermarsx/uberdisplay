package com.supermarsx.uberdisplay

import com.supermarsx.uberdisplay.media.DecoderController
import com.supermarsx.uberdisplay.protocol.CodecConstants
import org.junit.Assert.assertEquals
import org.junit.Test

class DecoderControllerTest {
    @Test
    fun codecIdToMimeReturnsAvcForH264() {
        assertEquals("video/avc", DecoderController.codecIdToMime(CodecConstants.CODEC_ID_H264))
    }

    @Test
    fun codecIdToMimeReturnsHevcForH265() {
        assertEquals("video/hevc", DecoderController.codecIdToMime(CodecConstants.CODEC_ID_H265))
    }

    @Test
    fun codecIdToMimeReturnsAv01ForAv1() {
        assertEquals("video/av01", DecoderController.codecIdToMime(CodecConstants.CODEC_ID_AV1))
    }

    @Test
    fun codecIdToMimeReturnsVp9ForVp9() {
        assertEquals("video/x-vnd.on2.vp9", DecoderController.codecIdToMime(CodecConstants.CODEC_ID_VP9))
    }

    @Test
    fun codecIdToMimeReturnsEvcForEvc() {
        assertEquals("video/evc", DecoderController.codecIdToMime(CodecConstants.CODEC_ID_EVC))
    }

    @Test
    fun codecIdToMimeReturnsLcevcForLcevc() {
        assertEquals("video/lcevc", DecoderController.codecIdToMime(CodecConstants.CODEC_ID_LCEVC))
    }

    @Test
    fun codecIdToMimeReturnsVvcForH266() {
        assertEquals("video/vvc", DecoderController.codecIdToMime(CodecConstants.CODEC_ID_H266))
    }

    @Test
    fun codecIdToMimeDefaultsToAvc() {
        assertEquals("video/avc", DecoderController.codecIdToMime(999))
    }

    @Test
    fun defaultStatusHasH264Codec() {
        val controller = DecoderController()
        val status = controller.getStatus()
        assertEquals(CodecConstants.CODEC_ID_H264, status.codecId)
        assertEquals("video/avc", status.mimeType)
        assertEquals(false, status.surfaceBound)
        assertEquals(false, status.decoderStarted)
    }

    @Test
    fun onFrameIgnoresEmptyData() {
        val controller = DecoderController()
        controller.onFrame(byteArrayOf())
        val status = controller.getStatus()
        assertEquals(false, status.decoderStarted)
    }
}
