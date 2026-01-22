use crate::vdd_protocol::{DriverManagerStatus, ServiceRequest, ServiceResponse};

#[cfg(windows)]
use windows::Win32::Storage::FileSystem::{
    CreateFileW, ReadFile, WriteFile, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
    FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};
#[cfg(windows)]
use windows::Win32::Foundation::{CloseHandle, HANDLE};
#[cfg(windows)]
use windows::core::PCWSTR;

const SERVICE_PIPE: &str = r"\\.\pipe\UberDisplayVddService";

pub fn status() -> Result<DriverManagerStatus, String> {
    let response = send(ServiceRequest { command: "status".to_string() })?;
    response.status.ok_or_else(|| "Service did not return status".to_string())
}

pub fn action(action: &str) -> Result<(), String> {
    let response = send(ServiceRequest { command: action.to_string() })?;
    if response.ok {
        Ok(())
    } else {
        Err(response.message)
    }
}

#[cfg(windows)]
fn send(request: ServiceRequest) -> Result<ServiceResponse, String> {
    let payload = serde_json::to_vec(&request).map_err(|err| err.to_string())?;
    let pipe = open_pipe()?;
    let result = write_pipe(pipe, &payload)
        .and_then(|_| read_pipe(pipe))
        .and_then(|response| serde_json::from_slice(&response).map_err(|err| err.to_string()));
    unsafe {
        let _ = CloseHandle(pipe);
    }
    result
}

#[cfg(not(windows))]
fn send(_request: ServiceRequest) -> Result<ServiceResponse, String> {
    Err("Virtual driver manager is only available on Windows.".to_string())
}

#[cfg(windows)]
fn open_pipe() -> Result<HANDLE, String> {
    let wide = to_wide(SERVICE_PIPE);
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
    .map_err(|err| format!("VDD service pipe unavailable: 0x{:08x}", err.code().0))?;
    if handle.is_invalid() {
        return Err("VDD service pipe unavailable".to_string());
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
        Err("Failed to write to VDD service pipe".to_string())
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
        Err("Failed to read from VDD service pipe".to_string())
    }
}

#[cfg(windows)]
fn to_wide(value: &str) -> Vec<u16> {
    let mut wide: Vec<u16> = value.encode_utf16().collect();
    wide.push(0);
    wide
}
