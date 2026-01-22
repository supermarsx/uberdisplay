use std::fs;
use std::path::PathBuf;

use crate::app_state::HostSettings;

const SETTINGS_FILE: &str = "host_settings.json";

pub fn load_settings(app_handle: &tauri::AppHandle) -> HostSettings {
    let path = settings_path(app_handle);
    let data = match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(_) => return HostSettings::default(),
    };

    serde_json::from_str(&data).unwrap_or_else(|_| HostSettings::default())
}

pub fn save_settings(app_handle: &tauri::AppHandle, settings: &HostSettings) -> Result<(), String> {
    let path = settings_path(app_handle);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    let payload = serde_json::to_string_pretty(settings).map_err(|err| err.to_string())?;
    fs::write(path, payload).map_err(|err| err.to_string())
}

fn settings_path(app_handle: &tauri::AppHandle) -> PathBuf {
    app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
        .join(SETTINGS_FILE)
}
