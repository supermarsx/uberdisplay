#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_state;
mod device_registry;
mod driver_probe;
mod transport_probe;
mod protocol;

#[tauri::command]
fn app_status(app_handle: tauri::AppHandle) -> app_state::AppStatus {
    let mut status = app_state::AppStatus::default();
    status.driver = driver_probe::probe_driver_status();
    status.transport = transport_probe::probe_transport_status();
    status.devices = device_registry::load_devices(&app_handle);
    status
}

#[tauri::command]
fn list_devices(app_handle: tauri::AppHandle) -> Vec<app_state::PairedDevice> {
    device_registry::load_devices(&app_handle)
}

#[tauri::command]
fn upsert_device(
    app_handle: tauri::AppHandle,
    device: app_state::PairedDevice,
) -> Result<Vec<app_state::PairedDevice>, String> {
    let mut devices = device_registry::load_devices(&app_handle);
    if let Some(existing) = devices.iter_mut().find(|item| item.id == device.id) {
        *existing = device;
    } else {
        devices.push(device);
    }
    device_registry::save_devices(&app_handle, &devices)?;
    Ok(devices)
}

#[tauri::command]
fn remove_device(app_handle: tauri::AppHandle, device_id: String) -> Result<Vec<app_state::PairedDevice>, String> {
    let mut devices = device_registry::load_devices(&app_handle);
    devices.retain(|device| device.id != device_id);
    device_registry::save_devices(&app_handle, &devices)?;
    Ok(devices)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app_status,
            list_devices,
            upsert_device,
            remove_device
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
