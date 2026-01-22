use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::app_state::{AppStatus, SessionStats};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DiagnosticsReport {
    timestamp: u64,
    app_status: AppStatus,
    session_stats: SessionStats,
}

pub fn export_report(
    app_handle: &tauri::AppHandle,
    status: AppStatus,
    stats: SessionStats,
) -> Result<String, String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| err.to_string())?
        .as_secs();
    let report = DiagnosticsReport {
        timestamp,
        app_status: status,
        session_stats: stats,
    };
    let payload = serde_json::to_string_pretty(&report).map_err(|err| err.to_string())?;

    let path = diagnostics_path(app_handle, timestamp);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    fs::write(&path, payload).map_err(|err| err.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

fn diagnostics_path(app_handle: &tauri::AppHandle, timestamp: u64) -> PathBuf {
    app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
        .join(format!("diagnostics_{timestamp}.json"))
}
