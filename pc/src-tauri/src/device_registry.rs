use std::fs;
use std::path::PathBuf;

use crate::app_state::PairedDevice;

const REGISTRY_FILE: &str = "paired_devices.json";

pub fn load_devices(app_handle: &tauri::AppHandle) -> Vec<PairedDevice> {
    let path = registry_path(app_handle);
    let data = match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(_) => return Vec::new(),
    };

    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

pub fn save_devices(app_handle: &tauri::AppHandle, devices: &[PairedDevice]) -> Result<(), String> {
    let path = registry_path(app_handle);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    let payload = serde_json::to_string_pretty(devices).map_err(|err| err.to_string())?;
    fs::write(path, payload).map_err(|err| err.to_string())
}

fn registry_path(app_handle: &tauri::AppHandle) -> PathBuf {
    app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
        .join(REGISTRY_FILE)
}
