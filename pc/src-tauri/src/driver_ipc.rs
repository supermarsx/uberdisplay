use serde_json::{json, Value};

#[cfg(windows)]
use windows::Win32::Storage::FileSystem::{
    CreateFileW, ReadFile, WriteFile, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
    FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};
#[cfg(windows)]
use windows::Win32::Foundation::{CloseHandle, HANDLE};
#[cfg(windows)]
use windows::core::PCWSTR;

const PIPE_NAME: &str = r"\\.\pipe\UberDisplayDriver";

pub fn create_display(label: &str) -> Result<(), String> {
    let payload = json!({ "label": label });
    let _ = send_command("create_display", payload)?;
    Ok(())
}

pub fn remove_display(display_id: &str) -> Result<(), String> {
    let payload = json!({ "displayId": display_id });
    let _ = send_command("remove_display", payload)?;
    Ok(())
}

pub fn list_modes(display_id: &str) -> Result<Value, String> {
    let payload = json!({ "displayId": display_id });
    send_command("list_modes", payload)
}

#[cfg(windows)]
fn send_command(command: &str, payload: Value) -> Result<Value, String> {
    let message = json!({
        "command": command,
        "payload": payload
    });
    let bytes = serde_json::to_vec(&message).map_err(|err| err.to_string())?;

    let pipe = open_pipe()?;
    let result = write_pipe(pipe, &bytes)
        .and_then(|_| read_pipe(pipe))
        .and_then(|response| serde_json::from_slice(&response).map_err(|err| err.to_string()));

    unsafe {
        CloseHandle(pipe);
    }

    result
}

#[cfg(not(windows))]
fn send_command(_command: &str, _payload: Value) -> Result<Value, String> {
    Err("Driver IPC is only available on Windows.".to_string())
}

#[cfg(windows)]
fn open_pipe() -> Result<HANDLE, String> {
    let wide = to_wide(PIPE_NAME);
    let handle = unsafe {
        CreateFileW(
            PCWSTR::from_raw(wide.as_ptr()),
            (FILE_GENERIC_READ | FILE_GENERIC_WRITE).0 as u32,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            None,
        )
    }
    .map_err(|err| format!("Driver IPC pipe unavailable: 0x{:08x}", err.code().0))?;
    if handle.is_invalid() {
        return Err("Driver IPC pipe unavailable".to_string());
    }
    Ok(handle)
}

#[cfg(windows)]
fn write_pipe(handle: HANDLE, data: &[u8]) -> Result<(), String> {
    let mut written = 0u32;
    let ok = unsafe { WriteFile(handle, Some(data), Some(&mut written), None) };
    if ok.is_ok() && written as usize == data.len() {
        Ok(())
    } else {
        Err("Failed to write to driver IPC pipe".to_string())
    }
}

#[cfg(windows)]
fn read_pipe(handle: HANDLE) -> Result<Vec<u8>, String> {
    let mut buffer = vec![0u8; 4096];
    let mut read = 0u32;
    let ok = unsafe { ReadFile(handle, Some(buffer.as_mut_slice()), Some(&mut read), None) };
    if ok.is_ok() && read > 0 {
        buffer.truncate(read as usize);
        Ok(buffer)
    } else {
        Err("Failed to read from driver IPC pipe".to_string())
    }
}

#[cfg(windows)]
fn to_wide(value: &str) -> Vec<u16> {
    let mut wide: Vec<u16> = value.encode_utf16().collect();
    wide.push(0);
    wide
}
