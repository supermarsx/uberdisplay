#[cfg(windows)]
use windows::Win32::Storage::FileSystem::{
    CreateFileW, ReadFile, WriteFile, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
    FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};
#[cfg(windows)]
use windows::Win32::Foundation::{CloseHandle, HANDLE};
#[cfg(windows)]
use windows::core::PCWSTR;

const PIPE_NAME: &str = r"\\.\pipe\MTTVirtualDisplayPipe";

pub fn set_display_count(count: u32) -> Result<(), String> {
    send_command(&format!("SETDISPLAYCOUNT {count}"), false)?;
    Ok(())
}

#[allow(dead_code)]
pub fn reload_driver() -> Result<(), String> {
    send_command("RELOAD_DRIVER", false)?;
    Ok(())
}

pub fn set_logging(enabled: bool) -> Result<(), String> {
    send_command(&format!("LOGGING {}", bool_to_str(enabled)), false)?;
    Ok(())
}

pub fn set_debug_logging(enabled: bool) -> Result<(), String> {
    send_command(&format!("LOG_DEBUG {}", bool_to_str(enabled)), false)?;
    Ok(())
}

pub fn set_hdr_plus(enabled: bool) -> Result<(), String> {
    send_command(&format!("HDRPLUS {}", bool_to_str(enabled)), false)?;
    Ok(())
}

pub fn set_sdr10(enabled: bool) -> Result<(), String> {
    send_command(&format!("SDR10 {}", bool_to_str(enabled)), false)?;
    Ok(())
}

pub fn set_custom_edid(enabled: bool) -> Result<(), String> {
    send_command(&format!("CUSTOMEDID {}", bool_to_str(enabled)), false)?;
    Ok(())
}

pub fn set_prevent_spoof(enabled: bool) -> Result<(), String> {
    send_command(&format!("PREVENTSPOOF {}", bool_to_str(enabled)), false)?;
    Ok(())
}

pub fn set_cea_override(enabled: bool) -> Result<(), String> {
    send_command(&format!("CEAOVERRIDE {}", bool_to_str(enabled)), false)?;
    Ok(())
}

pub fn set_hardware_cursor(enabled: bool) -> Result<(), String> {
    send_command(&format!("HARDWARECURSOR {}", bool_to_str(enabled)), false)?;
    Ok(())
}

pub fn set_gpu(name: &str) -> Result<(), String> {
    let sanitized = name.replace('"', "'");
    send_command(&format!("SETGPU \"{sanitized}\""), false)?;
    Ok(())
}

pub fn ping() -> Result<bool, String> {
    let response = send_command("PING", true)?;
    let Some(bytes) = response else {
        return Ok(false);
    };
    let text = decode_response(&bytes);
    Ok(text.to_ascii_uppercase().contains("PONG"))
}

pub fn get_settings_raw() -> Result<Option<String>, String> {
    let response = send_command("GETSETTINGS", true)?;
    Ok(response.map(|bytes| decode_response(&bytes)))
}

#[cfg(windows)]
fn send_command(command: &str, expect_response: bool) -> Result<Option<Vec<u8>>, String> {
    let bytes = to_wide_bytes(command);

    let pipe = open_pipe()?;
    let result = write_pipe(pipe, &bytes).and_then(|_| {
        if expect_response {
            read_pipe(pipe).map(Some)
        } else {
            Ok(None)
        }
    });

    unsafe {
        let _ = CloseHandle(pipe);
    }

    result
}

#[cfg(not(windows))]
fn send_command(_command: &str, _expect_response: bool) -> Result<Option<Vec<u8>>, String> {
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
    let mut buffer = vec![0u8; 1024];
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

#[cfg(windows)]
fn to_wide_bytes(value: &str) -> Vec<u8> {
    let wide: Vec<u16> = value.encode_utf16().collect();
    let mut bytes = Vec::with_capacity(wide.len() * 2);
    for item in wide {
        bytes.extend_from_slice(&item.to_le_bytes());
    }
    bytes
}

#[cfg(windows)]
fn decode_response(bytes: &[u8]) -> String {
    if bytes.len() >= 2 && bytes.len() % 2 == 0 {
        let utf16: Vec<u16> = bytes
            .chunks(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();
        if let Ok(value) = String::from_utf16(&utf16) {
            let trimmed = value.trim_matches('\0').trim().to_string();
            if !trimmed.is_empty() {
                return trimmed;
            }
        }
    }
    String::from_utf8_lossy(bytes)
        .trim_matches('\0')
        .trim()
        .to_string()
}

fn bool_to_str(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
