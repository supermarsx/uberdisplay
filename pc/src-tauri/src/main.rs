#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_state;
mod protocol;

#[tauri::command]
fn app_status() -> app_state::AppStatus {
    // TODO: Populate status from transport and driver probes.
    app_state::AppStatus::default()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![app_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
