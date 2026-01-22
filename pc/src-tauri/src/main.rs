#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_state;
mod diagnostics_report;
mod codec;
mod capture;
mod display_probe;
mod driver_manager;
mod driver_ipc;
mod linux_vdd;
mod service_manager;
mod vdd_protocol;
mod encoder;
mod device_registry;
mod driver_probe;
mod host_log;
mod host_transport;
mod mf_encoder;
mod session;
mod session_state;
mod stream_loop;
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
    status.session = app_state::SessionOverview {
        lifecycle: session_state::lifecycle_snapshot(),
    };
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
        session_state::update_active_device(Some(device.id.clone()), device.input_permissions.clone());
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
fn list_displays() -> Vec<app_state::DisplayInfo> {
    display_probe::list_displays()
}

#[tauri::command]
fn list_virtual_displays() -> Vec<app_state::DisplayInfo> {
    display_probe::list_displays()
        .into_iter()
        .filter(|display| display.is_virtual)
        .collect()
}

#[tauri::command]
fn virtual_display_count() -> u32 {
    #[cfg(windows)]
    {
        return display_probe::list_displays()
            .into_iter()
            .filter(|display| display.is_virtual)
            .count() as u32;
    }
    #[cfg(target_os = "linux")]
    {
        return linux_vdd::current_count();
    }
    #[cfg(not(any(windows, target_os = "linux")))]
    {
        0
    }
}

#[tauri::command]
fn virtual_driver_status(app_handle: tauri::AppHandle) -> Result<crate::vdd_protocol::DriverManagerStatus, String> {
    driver_manager::status().map_err(|err| {
        let _ = host_log::append_log(&app_handle, format!("Driver manager status failed: {err}"));
        err
    })
}

#[tauri::command]
fn virtual_driver_action(app_handle: tauri::AppHandle, action: String) -> Result<(), String> {
    driver_manager::action(&action).map_err(|err| {
        let _ = host_log::append_log(&app_handle, format!("Driver manager action failed: {err}"));
        err
    })?;
    let _ = host_log::append_log(&app_handle, format!("Driver manager action: {action}"));
    Ok(())
}

#[tauri::command]
fn set_virtual_display_count(app_handle: tauri::AppHandle, count: u32) -> Result<(), String> {
    #[cfg(windows)]
    {
        driver_ipc::set_display_count(count).map_err(|err| {
            let _ = host_log::append_log(&app_handle, format!("Driver IPC set display count failed: {err}"));
            err
        })?;
        let _ = host_log::append_log(&app_handle, format!("Requested {count} virtual displays"));
        return Ok(());
    }
    #[cfg(target_os = "linux")]
    {
        linux_vdd::ensure_display_count(count).map_err(|err| {
            let _ = host_log::append_log(&app_handle, format!("Linux VDD set display count failed: {err}"));
            err
        })?;
        let _ = host_log::append_log(&app_handle, format!("Requested {count} virtual displays"));
        return Ok(());
    }
    #[allow(unreachable_code)]
    Err("Virtual display management not supported on this platform.".to_string())
}

#[tauri::command]
fn set_linux_vdd_config(
    app_handle: tauri::AppHandle,
    base_display: u32,
    width: u32,
    height: u32,
    depth: u32,
) -> Result<(), String> {
    linux_vdd::set_config(base_display, width, height, depth).map_err(|err| {
        let _ = host_log::append_log(&app_handle, format!("Linux VDD config failed: {err}"));
        err
    })?;
    let _ = host_log::append_log(
        &app_handle,
        format!("Linux VDD config set: :{base_display} {width}x{height}x{depth}"),
    );
    Ok(())
}

#[tauri::command]
fn driver_pipe_ping(app_handle: tauri::AppHandle) -> Result<bool, String> {
    driver_ipc::ping().map_err(|err| {
        let _ = host_log::append_log(&app_handle, format!("Driver pipe ping failed: {err}"));
        err
    })
}

#[tauri::command]
fn driver_pipe_get_settings(app_handle: tauri::AppHandle) -> Result<Option<String>, String> {
    driver_ipc::get_settings_raw().map_err(|err| {
        let _ = host_log::append_log(&app_handle, format!("Driver pipe get settings failed: {err}"));
        err
    })
}

#[tauri::command]
fn driver_pipe_set_toggle(
    app_handle: tauri::AppHandle,
    option: String,
    enabled: bool,
) -> Result<(), String> {
    let result = match option.as_str() {
        "logging" => driver_ipc::set_logging(enabled),
        "debug" => driver_ipc::set_debug_logging(enabled),
        "hdr_plus" => driver_ipc::set_hdr_plus(enabled),
        "sdr10" => driver_ipc::set_sdr10(enabled),
        "custom_edid" => driver_ipc::set_custom_edid(enabled),
        "prevent_spoof" => driver_ipc::set_prevent_spoof(enabled),
        "cea_override" => driver_ipc::set_cea_override(enabled),
        "hardware_cursor" => driver_ipc::set_hardware_cursor(enabled),
        _ => Err("Unknown driver toggle option".to_string()),
    };
    result.map_err(|err| {
        let _ = host_log::append_log(&app_handle, format!("Driver pipe toggle failed: {err}"));
        err
    })?;
    let _ = host_log::append_log(&app_handle, format!("Driver toggle {option} set to {enabled}"));
    Ok(())
}

#[tauri::command]
fn driver_pipe_set_gpu(app_handle: tauri::AppHandle, name: String) -> Result<(), String> {
    driver_ipc::set_gpu(&name).map_err(|err| {
        let _ = host_log::append_log(&app_handle, format!("Driver pipe set GPU failed: {err}"));
        err
    })?;
    let _ = host_log::append_log(&app_handle, format!("Driver GPU set: {name}"));
    Ok(())
}

#[tauri::command]
fn install_vdd_service(app_handle: tauri::AppHandle) -> Result<(), String> {
    service_manager::install_service().map_err(|err| {
        let _ = host_log::append_log(&app_handle, format!("Service install failed: {err}"));
        err
    })?;
    let _ = host_log::append_log(&app_handle, "VDD service install requested");
    Ok(())
}

#[tauri::command]
fn start_vdd_service(app_handle: tauri::AppHandle) -> Result<(), String> {
    service_manager::start_service().map_err(|err| {
        let _ = host_log::append_log(&app_handle, format!("Service start failed: {err}"));
        err
    })?;
    let _ = host_log::append_log(&app_handle, "VDD service start requested");
    Ok(())
}

#[tauri::command]
fn stop_vdd_service(app_handle: tauri::AppHandle) -> Result<(), String> {
    service_manager::stop_service().map_err(|err| {
        let _ = host_log::append_log(&app_handle, format!("Service stop failed: {err}"));
        err
    })?;
    let _ = host_log::append_log(&app_handle, "VDD service stop requested");
    Ok(())
}

#[tauri::command]
fn query_vdd_service(app_handle: tauri::AppHandle) -> Result<String, String> {
    service_manager::query_service().map_err(|err| {
        let _ = host_log::append_log(&app_handle, format!("Service query failed: {err}"));
        err
    })
}
#[tauri::command]
fn list_display_modes(_app_handle: tauri::AppHandle, display_id: String) -> Vec<app_state::DisplayMode> {
    display_probe::list_display_modes(&display_id)
}

#[tauri::command]
fn create_virtual_display(app_handle: tauri::AppHandle, label: String) -> Result<(), String> {
    #[cfg(windows)]
    {
        let virtual_count = display_probe::list_displays()
            .into_iter()
            .filter(|display| display.is_virtual)
            .count() as u32;
        let next_count = virtual_count.saturating_add(1);
        driver_ipc::set_display_count(next_count).map_err(|err| {
            let _ = host_log::append_log(&app_handle, format!("Driver IPC set display count failed: {err}"));
            err
        })?;
        let _ = host_log::append_log(&app_handle, format!("Requested {next_count} virtual displays ({label})"));
        return Ok(());
    }
    #[cfg(target_os = "linux")]
    {
        let next_count = linux_vdd::current_count().saturating_add(1);
        linux_vdd::ensure_display_count(next_count).map_err(|err| {
            let _ = host_log::append_log(&app_handle, format!("Linux VDD start failed: {err}"));
            err
        })?;
        let _ = host_log::append_log(&app_handle, format!("Requested {next_count} virtual displays ({label})"));
        return Ok(());
    }
    #[allow(unreachable_code)]
    Err("Virtual display management not supported on this platform.".to_string())
}

#[tauri::command]
fn remove_virtual_display(app_handle: tauri::AppHandle, display_id: String) -> Result<(), String> {
    #[cfg(windows)]
    {
        let virtual_count = display_probe::list_displays()
            .into_iter()
            .filter(|display| display.is_virtual)
            .count() as u32;
        let next_count = virtual_count.saturating_sub(1);
        driver_ipc::set_display_count(next_count).map_err(|err| {
            let _ = host_log::append_log(&app_handle, format!("Driver IPC set display count failed: {err}"));
            err
        })?;
        let _ = host_log::append_log(&app_handle, format!("Requested {next_count} virtual displays (remove {display_id})"));
        return Ok(());
    }
    #[cfg(target_os = "linux")]
    {
        let next_count = linux_vdd::current_count().saturating_sub(1);
        linux_vdd::ensure_display_count(next_count).map_err(|err| {
            let _ = host_log::append_log(&app_handle, format!("Linux VDD stop failed: {err}"));
            err
        })?;
        let _ = host_log::append_log(&app_handle, format!("Requested {next_count} virtual displays (remove {display_id})"));
        return Ok(());
    }
    #[allow(unreachable_code)]
    Err("Virtual display management not supported on this platform.".to_string())
}

#[tauri::command]
fn set_session_display_target(app_handle: tauri::AppHandle, display_id: Option<String>) -> Result<(), String> {
    session_state::update_display_target(display_id.clone());
    let label = display_id.unwrap_or_else(|| "Auto".to_string());
    let _ = host_log::append_log(&app_handle, format!("Display target set: {label}"));
    Ok(())
}

#[tauri::command]
fn export_logs(app_handle: tauri::AppHandle) -> Result<String, String> {
    let path = host_log::export_logs(&app_handle)?;
    let _ = host_log::append_log(&app_handle, "Exported logs");
    Ok(path)
}

#[tauri::command]
fn clear_logs(app_handle: tauri::AppHandle) -> Result<(), String> {
    host_log::clear_logs(&app_handle)?;
    let _ = host_log::append_log(&app_handle, "Cleared logs");
    Ok(())
}

#[tauri::command]
fn export_diagnostics(app_handle: tauri::AppHandle) -> Result<String, String> {
    let status = app_status(app_handle.clone());
    let stats = session_state::stats_snapshot();
    let path = diagnostics_report::export_report(&app_handle, status, stats)?;
    let _ = host_log::append_log(&app_handle, "Exported diagnostics report");
    Ok(path)
}

#[tauri::command]
fn start_session(app_handle: tauri::AppHandle) -> Result<(), String> {
    let state = session_state::snapshot();
    let codec_id = state.codec_id.ok_or_else(|| "No negotiated codec".to_string())?;
    let config = session_state::config_snapshot().ok_or_else(|| "No session config".to_string())?;
    let display_target_id = state.display_target_id.clone();
    let settings = settings_registry::load_settings(&app_handle);
    let fps = settings.refresh_cap_hz.max(1) as u32;
    let bitrate_kbps = (settings.quality as u32 * 80).max(500);
    let keyframe_interval = settings.keyframe_interval.max(1) as u32;
    stream_loop::start_streaming(
        codec_id,
        config.encoder_id,
        config.width,
        config.height,
        bitrate_kbps,
        fps,
        keyframe_interval,
        display_target_id,
    )?;
    session_state::update_lifecycle(app_state::SessionLifecycle::Streaming);
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
    })
    .map_err(|err| {
        session_state::update_lifecycle(app_state::SessionLifecycle::Error);
        err
    })?;
    if let Some(codec_id) = codec::codec_id_from_name(&result.selection.codec_name) {
        session_state::update_codec(codec_id);
    }
    session_state::update_config(width, height, encoder_id);
    session_state::update_lifecycle(app_state::SessionLifecycle::Configured);
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
    session_state::update_lifecycle(app_state::SessionLifecycle::Connecting);
    if let Err(err) = host_transport::connect(&host, port) {
        session_state::update_lifecycle(app_state::SessionLifecycle::Error);
        return Err(err);
    }

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
    })
    .map_err(|err| {
        session_state::update_lifecycle(app_state::SessionLifecycle::Error);
        err
    })?;
    if let Some(codec_id) = codec::codec_id_from_name(&result.selection.codec_name) {
        session_state::update_codec(codec_id);
    }
    session_state::update_config(width, height, encoder_id);
    host_transport::send_framed_packet(&result.configure_bytes)?;
    host_transport::set_last_session(
        host,
        port,
        caps_packet.clone(),
        result.configure_bytes.clone(),
    );
    session_state::update_lifecycle(app_state::SessionLifecycle::Configured);
    let backend = encoder::select_backend(None);
    session_state::update_backend(backend);
    Ok(result.selection)
}

#[tauri::command]
fn tcp_disconnect() -> Result<(), String> {
    host_transport::disconnect()?;
    session_state::update_lifecycle(app_state::SessionLifecycle::Idle);
    Ok(())
}

#[tauri::command]
fn tcp_poll_status() -> (Option<u32>, Option<i32>) {
    (
        host_transport::take_last_client_codec_mask(),
        host_transport::take_last_frame_done(),
    )
}

#[tauri::command]
fn session_state_snapshot() -> (Option<u8>, Option<String>) {
    let snapshot = session_state::snapshot();
    let codec_id = snapshot.codec_id.map(|codec| codec as u8);
    let backend = snapshot.encoder_backend.map(|backend| format!("{backend:?}"));
    (codec_id, backend)
}

#[tauri::command]
fn session_stats_snapshot() -> app_state::SessionStats {
    session_state::stats_snapshot()
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

#[tauri::command]
fn set_device_input_permissions(
    app_handle: tauri::AppHandle,
    device_id: String,
    permissions: app_state::InputPermissions,
) -> Result<Vec<app_state::PairedDevice>, String> {
    let mut devices = device_registry::load_devices(&app_handle);
    if let Some(device) = devices.iter_mut().find(|item| item.id == device_id) {
        device.input_permissions = permissions.clone();
        let _ = host_log::append_log(&app_handle, format!("Updated input permissions for {}", device.name));
        session_state::update_active_device(Some(device.id.clone()), permissions);
    }
    device_registry::save_devices(&app_handle, &devices)?;
    Ok(devices)
}

#[tauri::command]
fn set_session_input_permissions(
    permissions: app_state::InputPermissions,
) -> Result<(), String> {
    session_state::update_input_permissions(permissions);
    Ok(())
}

#[tauri::command]
fn stop_session(app_handle: tauri::AppHandle) -> Result<(), String> {
    stream_loop::stop_streaming();
    let lifecycle = if host_transport::is_connected() {
        app_state::SessionLifecycle::Configured
    } else {
        app_state::SessionLifecycle::Idle
    };
    session_state::update_lifecycle(lifecycle);
    let _ = host_log::append_log(&app_handle, "Stop session requested");
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
            clear_logs,
            export_diagnostics,
            list_displays,
            list_virtual_displays,
            list_display_modes,
            virtual_driver_status,
            virtual_driver_action,
            virtual_display_count,
            set_virtual_display_count,
            set_linux_vdd_config,
            driver_pipe_ping,
            driver_pipe_get_settings,
            driver_pipe_set_toggle,
            driver_pipe_set_gpu,
            install_vdd_service,
            start_vdd_service,
            stop_vdd_service,
            query_vdd_service,
            create_virtual_display,
            remove_virtual_display,
            set_session_display_target,
            start_session,
            prepare_session,
            tcp_connect_and_configure,
            tcp_disconnect,
            tcp_poll_status,
            session_state_snapshot,
            session_stats_snapshot,
            add_virtual_display,
            record_action,
            set_device_input_permissions,
            set_session_input_permissions,
            stop_session
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
