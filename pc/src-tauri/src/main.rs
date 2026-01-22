#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_state;
mod codec;
mod encoder;
mod device_registry;
mod driver_probe;
mod host_log;
mod host_transport;
mod session;
mod transport_probe;
mod settings_registry;
mod protocol;

#[tauri::command]
fn app_status(app_handle: tauri::AppHandle) -> app_state::AppStatus {
    let mut status = app_state::AppStatus::default();
    status.driver = driver_probe::probe_driver_status();
    status.transport = transport_probe::probe_transport_status();
    status.devices = device_registry::load_devices(&app_handle);
    status.settings = settings_registry::load_settings(&app_handle);
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
    let _ = host_log::append_log(&app_handle, format!("Removed device {}", device_id));
    Ok(devices)
}

#[tauri::command]
fn connect_device(app_handle: tauri::AppHandle, device_id: String) -> Result<Vec<app_state::PairedDevice>, String> {
    let mut devices = device_registry::load_devices(&app_handle);
    if let Some(device) = devices.iter_mut().find(|item| item.id == device_id) {
        device.status = "Connected".to_string();
        device.last_seen = Some("Just now".to_string());
        let _ = host_log::append_log(&app_handle, format!("Connected to {}", device.name));
    }
    device_registry::save_devices(&app_handle, &devices)?;
    Ok(devices)
}

#[tauri::command]
fn update_settings(
    app_handle: tauri::AppHandle,
    settings: app_state::HostSettings,
) -> Result<app_state::HostSettings, String> {
    let codec_id =
        codec::codec_id_from_name(&settings.codec).ok_or_else(|| "Unsupported codec".to_string())?;
    let host_mask = codec::host_codec_mask();
    if codec::codec_mask(codec_id) & host_mask == 0 {
        return Err("Codec not available on this host".to_string());
    }
    settings_registry::save_settings(&app_handle, &settings)?;
    let _ = host_log::append_log(&app_handle, "Updated host settings");
    Ok(settings)
}

#[tauri::command]
fn reset_settings(app_handle: tauri::AppHandle) -> Result<app_state::HostSettings, String> {
    let settings = app_state::HostSettings::default();
    settings_registry::save_settings(&app_handle, &settings)?;
    let _ = host_log::append_log(&app_handle, "Reset host settings to defaults");
    Ok(settings)
}

#[tauri::command]
fn negotiate_codec(
    app_handle: tauri::AppHandle,
    client_mask: u32,
) -> Result<app_state::CodecSelection, String> {
    let settings = settings_registry::load_settings(&app_handle);
    let preferred = codec::codec_id_from_name(&settings.codec);
    let host_mask = codec::host_codec_mask();
    let selected = codec::select_codec(host_mask, client_mask, preferred)
        .ok_or_else(|| "No compatible codec found".to_string())?;

    let selection = app_state::CodecSelection {
        codec_id: selected as u8,
        codec_name: codec::codec_name(selected).to_string(),
        host_mask,
        client_mask,
    };
    let _ = host_log::append_log(
        &app_handle,
        format!("Negotiated codec {}", selection.codec_name),
    );
    Ok(selection)
}

#[tauri::command]
fn list_logs(app_handle: tauri::AppHandle) -> Vec<host_log::HostLogEntry> {
    host_log::load_logs(&app_handle)
}

#[tauri::command]
fn export_logs(app_handle: tauri::AppHandle) -> Result<String, String> {
    let path = host_log::export_logs(&app_handle)?;
    let _ = host_log::append_log(&app_handle, "Exported logs");
    Ok(path)
}

#[tauri::command]
fn start_session(app_handle: tauri::AppHandle) -> Result<(), String> {
    let _ = host_log::append_log(&app_handle, "Start session requested");
    Ok(())
}

#[tauri::command]
fn prepare_session(
    app_handle: tauri::AppHandle,
    width: i32,
    height: i32,
    host_width: i32,
    host_height: i32,
    encoder_id: i32,
    client_codec_mask: u32,
) -> Result<(app_state::CodecSelection, Vec<u8>), String> {
    let settings = settings_registry::load_settings(&app_handle);
    let preferred = codec::codec_id_from_name(&settings.codec);
    let result = session::prepare_session(session::SessionConfig {
        width,
        height,
        host_width,
        host_height,
        encoder_id,
        client_codec_mask,
        preferred_codec: preferred,
    })?;
    let _ = host_log::append_log(
        &app_handle,
        format!("Prepared session codec {}", result.selection.codec_name),
    );
    Ok((result.selection, result.configure_bytes))
}

#[tauri::command]
fn tcp_connect_and_configure(
    app_handle: tauri::AppHandle,
    host: String,
    port: u16,
    width: i32,
    height: i32,
    host_width: i32,
    host_height: i32,
    encoder_id: i32,
    client_codec_mask: u32,
) -> Result<app_state::CodecSelection, String> {
    host_transport::connect(&host, port)?;

    let host_caps = protocol::packets::CapabilitiesPacket {
        codec_mask: codec::host_codec_mask(),
        flags: 0,
    };
    let caps_packet = protocol::packets::build_capabilities_packet(host_caps);
    host_transport::send_framed_packet(&caps_packet)?;

    let settings = settings_registry::load_settings(&app_handle);
    let preferred = codec::codec_id_from_name(&settings.codec);
    let result = session::prepare_session(session::SessionConfig {
        width,
        height,
        host_width,
        host_height,
        encoder_id,
        client_codec_mask,
        preferred_codec: preferred,
    })?;
    host_transport::send_framed_packet(&result.configure_bytes)?;
    Ok(result.selection)
}

#[tauri::command]
fn tcp_disconnect() -> Result<(), String> {
    host_transport::disconnect()
}

#[tauri::command]
fn add_virtual_display(app_handle: tauri::AppHandle) -> Result<(), String> {
    let _ = host_log::append_log(&app_handle, "Add virtual display requested");
    Ok(())
}

#[tauri::command]
fn record_action(app_handle: tauri::AppHandle, message: String) -> Result<(), String> {
    let _ = host_log::append_log(&app_handle, message);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app_status,
            list_devices,
            upsert_device,
            remove_device,
            connect_device,
            update_settings,
            reset_settings,
            negotiate_codec,
            list_logs,
            export_logs,
            start_session,
            prepare_session,
            tcp_connect_and_configure,
            tcp_disconnect,
            add_virtual_display,
            record_action
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
