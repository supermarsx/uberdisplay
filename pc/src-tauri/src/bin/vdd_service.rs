use std::ffi::OsString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use std::thread;

use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Storage::FileSystem::{ReadFile, WriteFile, PIPE_ACCESS_DUPLEX};
use windows::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeW, PIPE_READMODE_MESSAGE, PIPE_TYPE_MESSAGE,
    PIPE_UNLIMITED_INSTANCES, PIPE_WAIT,
};
use windows::core::PCWSTR;
use windows_service::service::{
    ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
    ServiceType,
};
use windows_service::service_control_handler::{self, ServiceControlHandlerResult};
use windows_service::{define_windows_service, service_dispatcher};

use uberdisplay_pc::vdd_ops;
use uberdisplay_pc::vdd_protocol::{ServiceRequest, ServiceResponse};

const SERVICE_NAME: &str = "UberDisplayVddService";
const PIPE_NAME: &str = r"\\.\pipe\UberDisplayVddService";

static RUNNING: OnceLock<AtomicBool> = OnceLock::new();

define_windows_service!(ffi_service_main, service_main);

fn main() -> windows_service::Result<()> {
    if std::env::args().any(|arg| arg == "--console") {
        run_server();
        return Ok(());
    }
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)
}

fn service_main(_arguments: Vec<OsString>) {
    let running = RUNNING.get_or_init(|| AtomicBool::new(true));
    running.store(true, Ordering::SeqCst);

    let status_handle = service_control_handler::register(SERVICE_NAME, move |control| {
        match control {
            ServiceControl::Stop => {
                running.store(false, Ordering::SeqCst);
                ServiceControlHandlerResult::NoError
            }
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    })
    .expect("Failed to register service control handler");

    let mut status = ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: std::time::Duration::from_secs(2),
        process_id: None,
    };

    status_handle
        .set_service_status(status.clone())
        .expect("Failed to set service status");

    run_server();

    status.current_state = ServiceState::Stopped;
    status_handle
        .set_service_status(status)
        .expect("Failed to set service status");
}

fn run_server() {
    let running = RUNNING.get_or_init(|| AtomicBool::new(true));
    running.store(true, Ordering::SeqCst);
    while running.load(Ordering::SeqCst) {
        match create_pipe() {
            Ok(pipe) => {
                let connected = unsafe { ConnectNamedPipe(pipe, None) }.is_ok();
                if connected {
                    handle_client(pipe);
                }
                unsafe {
                    CloseHandle(pipe);
                }
            }
            Err(_) => thread::sleep(std::time::Duration::from_millis(500)),
        }
    }
}

fn handle_client(pipe: HANDLE) {
    let mut buffer = vec![0u8; 4096];
    let mut read = 0u32;
    let ok = unsafe { ReadFile(pipe, Some(buffer.as_mut_slice()), Some(&mut read), None) };
    if !ok.is_ok() || read == 0 {
        return;
    }
    buffer.truncate(read as usize);

    let response = match serde_json::from_slice::<ServiceRequest>(&buffer) {
        Ok(request) => handle_request(request),
        Err(err) => ServiceResponse {
            ok: false,
            message: format!("Invalid request: {err}"),
            status: None,
        },
    };

    if let Ok(payload) = serde_json::to_vec(&response) {
        let mut written = 0u32;
        let _ = unsafe { WriteFile(pipe, Some(payload.as_slice()), Some(&mut written), None) };
    }
}

fn handle_request(request: ServiceRequest) -> ServiceResponse {
    match request.command.as_str() {
        "status" => match vdd_ops::status() {
            Ok(status) => ServiceResponse {
                ok: true,
                message: "ok".to_string(),
                status: Some(status),
            },
            Err(err) => ServiceResponse {
                ok: false,
                message: err,
                status: None,
            },
        },
        "enable" => map_action(vdd_ops::enable()),
        "disable" => map_action(vdd_ops::disable()),
        "toggle" => map_action(vdd_ops::toggle()),
        "install" => map_action(vdd_ops::install_from_vendor()),
        "uninstall" => map_action(vdd_ops::uninstall()),
        _ => ServiceResponse {
            ok: false,
            message: "Unknown command".to_string(),
            status: None,
        },
    }
}

fn map_action(result: Result<(), String>) -> ServiceResponse {
    match result {
        Ok(()) => ServiceResponse {
            ok: true,
            message: "ok".to_string(),
            status: None,
        },
        Err(err) => ServiceResponse {
            ok: false,
            message: err,
            status: None,
        },
    }
}

fn create_pipe() -> Result<HANDLE, String> {
    let wide = to_wide(PIPE_NAME);
    let handle = unsafe {
        CreateNamedPipeW(
            PCWSTR::from_raw(wide.as_ptr()),
            PIPE_ACCESS_DUPLEX,
            PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
            PIPE_UNLIMITED_INSTANCES,
            4096,
            4096,
            0,
            None,
        )
    };
    if handle.is_invalid() {
        return Err("Failed to create named pipe".to_string());
    }
    Ok(handle)
}

fn to_wide(value: &str) -> Vec<u16> {
    let mut wide: Vec<u16> = value.encode_utf16().collect();
    wide.push(0);
    wide
}
