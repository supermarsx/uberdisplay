use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriverManagerStatus {
    pub installed: bool,
    pub status: String,
    #[serde(default)]
    pub details: Option<DriverManagerDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriverManagerDetails {
    #[serde(default)]
    pub friendly_name: Option<String>,
    #[serde(default)]
    pub instance_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRequest {
    pub command: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceResponse {
    pub ok: bool,
    pub message: String,
    #[serde(default)]
    pub status: Option<DriverManagerStatus>,
}
