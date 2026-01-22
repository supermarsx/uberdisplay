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
    pub input_mode: String,
}

impl Default for HostSettings {
    fn default() -> Self {
        Self {
            codec: "H.264 High".to_string(),
            quality: 80,
            refresh_cap_hz: 120,
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
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodecSelection {
    pub codec_id: u8,
    pub codec_name: String,
    pub host_mask: u32,
    pub client_mask: u32,
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
        }
    }
}
