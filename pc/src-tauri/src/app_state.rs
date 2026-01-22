use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DriverStatus {
    pub installed: bool,
    pub active: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransportStatus {
    pub tcp_listening: bool,
    pub tcp_connections: u32,
    pub aoap_attached: bool,
}

#[derive(Debug, Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HostSettings {
    pub codec: String,
    pub quality: u8,
    pub refresh_cap_hz: u16,
    #[serde(default)]
    pub keyframe_interval: u16,
    pub input_mode: String,
}

impl Default for HostSettings {
    fn default() -> Self {
        Self {
            codec: "H.264 High".to_string(),
            quality: 80,
            refresh_cap_hz: 120,
            keyframe_interval: 60,
            input_mode: "Touch + Pen".to_string(),
        }
    }
}

#[derive(Debug, Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PairedDevice {
    pub id: String,
    pub name: String,
    pub transport: String,
    pub status: String,
    pub last_seen: Option<String>,
    #[serde(default)]
    pub input_permissions: InputPermissions,
}

#[derive(Debug, Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InputPermissions {
    pub enable_input: bool,
    pub touch: bool,
    pub pen: bool,
    pub keyboard: bool,
}

impl Default for InputPermissions {
    fn default() -> Self {
        Self {
            enable_input: true,
            touch: true,
            pen: true,
            keyboard: true,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStatus {
    pub protocol_version: u16,
    pub driver: DriverStatus,
    pub transport: TransportStatus,
    pub settings: HostSettings,
    pub devices: Vec<PairedDevice>,
    pub session: SessionOverview,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodecSelection {
    pub codec_id: u8,
    pub codec_name: String,
    pub host_mask: u32,
    pub client_mask: u32,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SessionLifecycle {
    Idle,
    Connecting,
    Configured,
    Streaming,
    Error,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionOverview {
    pub lifecycle: SessionLifecycle,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionStats {
    pub fps: f32,
    pub bitrate_kbps: u32,
    pub frames_sent: u64,
    pub frames_acked: u64,
    pub last_frame_bytes: u32,
    pub queue_depth: u32,
    pub dxgi_timeouts: u32,
    pub dxgi_access_lost: u32,
    pub dxgi_failures: u32,
    pub dxgi_last_bytes: u32,
    pub capture_path: String,
    pub capture_scale: String,
}

impl Default for SessionStats {
    fn default() -> Self {
        Self {
            fps: 0.0,
            bitrate_kbps: 0,
            frames_sent: 0,
            frames_acked: 0,
            last_frame_bytes: 0,
            queue_depth: 0,
            dxgi_timeouts: 0,
            dxgi_access_lost: 0,
            dxgi_failures: 0,
            dxgi_last_bytes: 0,
            capture_path: "Unknown".to_string(),
            capture_scale: "Unknown".to_string(),
        }
    }
}

impl Default for AppStatus {
    fn default() -> Self {
        Self {
            protocol_version: 4,
            driver: DriverStatus {
                installed: false,
                active: false,
            },
            transport: TransportStatus {
                tcp_listening: true,
                tcp_connections: 0,
                aoap_attached: false,
            },
            settings: HostSettings::default(),
            devices: Vec::new(),
            session: SessionOverview {
                lifecycle: SessionLifecycle::Idle,
            },
        }
    }
}
