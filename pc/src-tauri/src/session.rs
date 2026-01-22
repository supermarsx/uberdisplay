use crate::app_state::CodecSelection;
use crate::codec::{self, CodecId};
use crate::protocol::packets::{build_configure_packet, ConfigurePacket};

#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub width: i32,
    pub height: i32,
    pub host_width: i32,
    pub host_height: i32,
    pub encoder_id: i32,
    pub client_codec_mask: u32,
    pub preferred_codec: Option<CodecId>,
}

pub struct SessionPrepareResult {
    pub selection: CodecSelection,
    pub configure_bytes: Vec<u8>,
}

pub fn prepare_session(config: SessionConfig) -> Result<SessionPrepareResult, String> {
    let host_mask = codec::host_codec_mask();
    let selected = codec::select_codec(host_mask, config.client_codec_mask, config.preferred_codec)
        .ok_or_else(|| "No compatible codec found".to_string())?;

    let configure = ConfigurePacket {
        width: config.width,
        height: config.height,
        host_width: config.host_width,
        host_height: config.host_height,
        encoder_id: config.encoder_id,
        codec_id: Some(selected as u8),
        codec_profile: 0,
        codec_level: 0,
        codec_flags: 0,
    };

    Ok(SessionPrepareResult {
        selection: CodecSelection {
            codec_id: selected as u8,
            codec_name: codec::codec_name(selected).to_string(),
            host_mask,
            client_mask: config.client_codec_mask,
        },
        configure_bytes: build_configure_packet(configure),
    })
}
