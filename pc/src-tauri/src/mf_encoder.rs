use crate::codec::CodecId;

pub struct MfEncoder {
    pub codec_id: CodecId,
    pub width: i32,
    pub height: i32,
}

impl MfEncoder {
    pub fn new(codec_id: CodecId, width: i32, height: i32) -> Result<Self, String> {
        match codec_id {
            CodecId::H264 | CodecId::H265 => Ok(Self {
                codec_id,
                width,
                height,
            }),
            _ => Err("Media Foundation encoder supports H.264/H.265 only".to_string()),
        }
    }

    pub fn encode_dummy_frame(&self) -> Vec<u8> {
        // Placeholder until Media Foundation pipeline is implemented.
        Vec::new()
    }
}
