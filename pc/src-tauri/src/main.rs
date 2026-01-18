#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod protocol;

use serde::Serialize;

#[derive(Debug, Serialize)]
struct AppStatus {
    protocol_version: u16,
    driver_ready: bool,
    usb_ready: bool,
    wifi_ready: bool,
}

#[tauri::command]
fn app_status() -> AppStatus {
    // TODO: Populate status from transport and driver probes.
    AppStatus {
        protocol_version: 4,
        driver_ready: false,
        usb_ready: false,
        wifi_ready: true,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![app_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
