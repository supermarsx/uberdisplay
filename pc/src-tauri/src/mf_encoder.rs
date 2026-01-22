use crate::codec::CodecId;

pub struct MfEncoder {
    pub codec_id: CodecId,
    pub width: i32,
    pub height: i32,
    pub bitrate_kbps: u32,
    pub fps: u32,
    pub keyframe_interval: u32,
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
            }),
            _ => Err("Media Foundation encoder supports H.264/H.265 only".to_string()),
        }
    }

    pub fn encode_dummy_frame(&self) -> Vec<u8> {
        // Placeholder until Media Foundation pipeline is implemented.
        // Size approximates bitrate/fps to keep pacing behavior realistic.
        let fps = self.fps.max(1);
        let bytes_per_frame = ((self.bitrate_kbps as u64 * 1000) / 8 / fps as u64)
            .clamp(128, 64 * 1024) as usize;
        vec![0u8; bytes_per_frame]
    }
}
