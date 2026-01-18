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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HostSettings {
    pub codec: String,
    pub quality: u8,
    pub refresh_cap_hz: u16,
    pub input_mode: String,
}

#[derive(Debug, Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PairedDevice {
    pub id: String,
    pub name: String,
    pub transport: String,
    pub status: String,
    pub last_seen: Option<String>,
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
            settings: HostSettings {
                codec: "H.264 High".to_string(),
                quality: 80,
                refresh_cap_hz: 120,
                input_mode: "Touch + Pen".to_string(),
            },
            devices: Vec::new(),
        }
    }
}
