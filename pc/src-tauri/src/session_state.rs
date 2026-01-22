use std::sync::{Mutex, OnceLock};

use crate::codec::CodecId;
use crate::encoder::EncoderBackend;

#[derive(Debug, Clone)]
pub struct SessionState {
    pub codec_id: Option<CodecId>,
    pub encoder_backend: Option<EncoderBackend>,
}

static SESSION_STATE: OnceLock<Mutex<SessionState>> = OnceLock::new();

fn state_store() -> &'static Mutex<SessionState> {
    SESSION_STATE.get_or_init(|| Mutex::new(SessionState {
        codec_id: None,
        encoder_backend: None,
    }))
}

pub fn update_codec(codec_id: CodecId) {
    if let Ok(mut state) = state_store().lock() {
        state.codec_id = Some(codec_id);
    }
}

pub fn update_backend(backend: EncoderBackend) {
    if let Ok(mut state) = state_store().lock() {
        state.encoder_backend = Some(backend);
    }
}

pub fn snapshot() -> SessionState {
    state_store()
        .lock()
        .map(|state| state.clone())
        .unwrap_or(SessionState {
            codec_id: None,
            encoder_backend: None,
        })
}
