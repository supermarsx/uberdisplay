use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

const LOG_FILE: &str = "host_log.json";
const LOG_EXPORT_FILE: &str = "host_log.txt";
const LOG_LIMIT: usize = 200;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HostLogEntry {
    pub timestamp: u64,
    pub message: String,
}

pub fn load_logs(app_handle: &tauri::AppHandle) -> Vec<HostLogEntry> {
    let path = log_path(app_handle);
    let data = match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(_) => return Vec::new(),
    };

    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

pub fn append_log(app_handle: &tauri::AppHandle, message: impl Into<String>) -> Result<(), String> {
    let mut logs = load_logs(app_handle);
    logs.push(HostLogEntry {
        timestamp: now_timestamp(),
        message: message.into(),
    });
    if logs.len() > LOG_LIMIT {
        logs.drain(0..logs.len() - LOG_LIMIT);
    }
    save_logs(app_handle, &logs)
}

pub fn export_logs(app_handle: &tauri::AppHandle) -> Result<String, String> {
    let logs = load_logs(app_handle);
    let export_path = export_path(app_handle);
    if let Some(parent) = export_path.parent() {
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    let mut lines = Vec::new();
    for entry in logs {
        lines.push(format!("{}\t{}", entry.timestamp, entry.message));
    }
    fs::write(&export_path, lines.join("\n")).map_err(|err| err.to_string())?;
    Ok(export_path.to_string_lossy().to_string())
}

pub fn clear_logs(app_handle: &tauri::AppHandle) -> Result<(), String> {
    save_logs(app_handle, &[])
}

fn save_logs(app_handle: &tauri::AppHandle, logs: &[HostLogEntry]) -> Result<(), String> {
    let path = log_path(app_handle);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    let payload = serde_json::to_string_pretty(logs).map_err(|err| err.to_string())?;
    fs::write(path, payload).map_err(|err| err.to_string())
}

fn log_path(app_handle: &tauri::AppHandle) -> PathBuf {
    app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
        .join(LOG_FILE)
}

fn export_path(app_handle: &tauri::AppHandle) -> PathBuf {
    app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
        .join("logs")
        .join(LOG_EXPORT_FILE)
}

fn now_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
