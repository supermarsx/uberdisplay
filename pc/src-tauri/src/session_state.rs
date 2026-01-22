use std::sync::{Mutex, OnceLock};

use crate::app_state::SessionStats;
use crate::codec::CodecId;
use crate::encoder::EncoderBackend;

#[derive(Debug, Clone)]
pub struct SessionState {
    pub codec_id: Option<CodecId>,
    pub encoder_backend: Option<EncoderBackend>,
    pub active_device_id: Option<String>,
    pub input_permissions: crate::app_state::InputPermissions,
    pub stats: SessionStats,
}

static SESSION_STATE: OnceLock<Mutex<SessionState>> = OnceLock::new();

fn state_store() -> &'static Mutex<SessionState> {
    SESSION_STATE.get_or_init(|| Mutex::new(SessionState {
        codec_id: None,
        encoder_backend: None,
        active_device_id: None,
        input_permissions: crate::app_state::InputPermissions::default(),
        stats: SessionStats::default(),
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

pub fn update_active_device(device_id: Option<String>, permissions: crate::app_state::InputPermissions) {
    if let Ok(mut state) = state_store().lock() {
        state.active_device_id = device_id;
        state.input_permissions = permissions;
    }
}

pub fn update_input_permissions(permissions: crate::app_state::InputPermissions) {
    if let Ok(mut state) = state_store().lock() {
        state.input_permissions = permissions;
    }
}

pub fn update_stats(stats: SessionStats) {
    if let Ok(mut state) = state_store().lock() {
        state.stats = stats;
    }
}

pub fn reset_stats() {
    if let Ok(mut state) = state_store().lock() {
        state.stats = SessionStats::default();
    }
}

pub fn stats_snapshot() -> SessionStats {
    state_store()
        .lock()
        .map(|state| state.stats.clone())
        .unwrap_or_else(|_| SessionStats::default())
}

pub fn snapshot() -> SessionState {
    state_store()
        .lock()
        .map(|state| state.clone())
        .unwrap_or(SessionState {
            codec_id: None,
            encoder_backend: None,
            active_device_id: None,
            input_permissions: crate::app_state::InputPermissions::default(),
            stats: SessionStats::default(),
        })
}
