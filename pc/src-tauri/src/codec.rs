#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodecId {
    H264 = 1,
    H265 = 2,
    Av1 = 3,
    Vp9 = 4,
    H266 = 5,
}

pub const CODEC_MASK_H264: u32 = 1 << 0;
pub const CODEC_MASK_H265: u32 = 1 << 1;
pub const CODEC_MASK_AV1: u32 = 1 << 2;
pub const CODEC_MASK_VP9: u32 = 1 << 3;
pub const CODEC_MASK_H266: u32 = 1 << 4;

pub fn codec_id_from_name(name: &str) -> Option<CodecId> {
    match name.trim().to_ascii_lowercase().as_str() {
        "h.264" | "h.264 high" | "h264" => Some(CodecId::H264),
        "h.265" | "h.265 hevc" | "hevc" | "h265" => Some(CodecId::H265),
        "av1" => Some(CodecId::Av1),
        "vp9" => Some(CodecId::Vp9),
        "h.266" | "h266" => Some(CodecId::H266),
        _ => None,
    }
}

pub fn codec_name(codec_id: CodecId) -> &'static str {
    match codec_id {
        CodecId::H264 => "H.264",
        CodecId::H265 => "H.265 HEVC",
        CodecId::Av1 => "AV1",
        CodecId::Vp9 => "VP9",
        CodecId::H266 => "H.266",
    }
}

pub fn codec_mask(codec_id: CodecId) -> u32 {
    match codec_id {
        CodecId::H264 => CODEC_MASK_H264,
        CodecId::H265 => CODEC_MASK_H265,
        CodecId::Av1 => CODEC_MASK_AV1,
        CodecId::Vp9 => CODEC_MASK_VP9,
        CodecId::H266 => CODEC_MASK_H266,
    }
}

pub fn host_codec_mask() -> u32 {
    #[cfg(windows)]
    {
        CODEC_MASK_H265 | CODEC_MASK_AV1 | CODEC_MASK_H264 | CODEC_MASK_VP9
    }
    #[cfg(not(windows))]
    {
        CODEC_MASK_H264
    }
}

pub fn select_codec(
    host_mask: u32,
    client_mask: u32,
    preferred: Option<CodecId>,
) -> Option<CodecId> {
    let available = host_mask & client_mask;
    if let Some(codec) = preferred {
        if available & codec_mask(codec) != 0 {
            return Some(codec);
        }
    }

    let priority = [CodecId::H265, CodecId::Av1, CodecId::H264, CodecId::Vp9];
    for codec in priority {
        if available & codec_mask(codec) != 0 {
            return Some(codec);
        }
    }

    None
}
