use crate::codec::CodecId;

pub struct MfEncoder {
    pub codec_id: CodecId,
    pub width: i32,
    pub height: i32,
    pub bitrate_kbps: u32,
    pub fps: u32,
    pub keyframe_interval: u32,
    frame_index: u64,
}

impl MfEncoder {
    pub fn new(
        codec_id: CodecId,
        width: i32,
        height: i32,
        bitrate_kbps: u32,
        fps: u32,
        keyframe_interval: u32,
    ) -> Result<Self, String> {
        match codec_id {
            CodecId::H264 | CodecId::H265 => Ok(Self {
                codec_id,
                width,
                height,
                bitrate_kbps,
                fps,
                keyframe_interval,
                frame_index: 0,
            }),
            _ => Err("Media Foundation encoder supports H.264/H.265 only".to_string()),
        }
    }

    pub fn encode_dummy_frame(&mut self) -> Vec<u8> {
        // Placeholder until Media Foundation pipeline is implemented.
        // Size approximates bitrate/fps to keep pacing behavior realistic.
        let fps = self.fps.max(1);
        let bitrate_bytes = ((self.bitrate_kbps as u64 * 1000) / 8 / fps as u64)
            .clamp(128, 512 * 1024);
        let pixel_area = (self.width.max(1) as u64) * (self.height.max(1) as u64);
        let resolution_bytes = (pixel_area / 80).clamp(256, 512 * 1024);
        let mut bytes_per_frame = bitrate_bytes.max(resolution_bytes);
        if self.codec_id == CodecId::H265 {
            bytes_per_frame = bytes_per_frame.saturating_mul(9) / 10;
        }
        self.frame_index = self.frame_index.wrapping_add(1);
        if self.keyframe_interval > 0
            && self.frame_index % self.keyframe_interval as u64 == 0
        {
            bytes_per_frame = bytes_per_frame.saturating_mul(2).min(768 * 1024);
        }
        vec![0u8; bytes_per_frame as usize]
    }
}
