use crate::vdd_protocol::{DriverManagerDetails, DriverManagerStatus};

#[cfg(windows)]
use std::process::Command;

#[cfg(windows)]
#[derive(Debug, Clone)]
struct DeviceRecord {
    instance_id: String,
    description: String,
    status: String,
}

#[cfg(windows)]
pub fn status() -> Result<DriverManagerStatus, String> {
    let records = enum_display_devices()?;
    let found = records.into_iter().find(is_virtual_display_device);
    if let Some(device) = found {
        let enabled = is_status_enabled(&device.status);
        let status = if enabled { "enabled" } else { "disabled" }.to_string();
        Ok(DriverManagerStatus {
            installed: true,
            status,
            details: Some(DriverManagerDetails {
                friendly_name: Some(device.description),
                instance_id: Some(device.instance_id),
            }),
        })
    } else {
        Ok(DriverManagerStatus {
            installed: false,
            status: "not installed".to_string(),
            details: None,
        })
    }
}

#[cfg(windows)]
pub fn enable() -> Result<(), String> {
    let device = find_device()?;
    run_pnputil(&["/enable-device", &device.instance_id])
}

#[cfg(windows)]
pub fn disable() -> Result<(), String> {
    let device = find_device()?;
    run_pnputil(&["/disable-device", &device.instance_id])
}

#[cfg(windows)]
pub fn toggle() -> Result<(), String> {
    let device = find_device()?;
    if is_status_enabled(&device.status) {
        run_pnputil(&["/disable-device", &device.instance_id])
    } else {
        run_pnputil(&["/enable-device", &device.instance_id])
    }
}

#[cfg(windows)]
pub fn install_from_vendor() -> Result<(), String> {
    let inf = find_vendor_inf()?;
    run_pnputil(&["/add-driver", inf.as_str(), "/install"])
}

#[cfg(windows)]
pub fn uninstall() -> Result<(), String> {
    let device = find_device()?;
    run_pnputil(&["/remove-device", &device.instance_id])
}

#[cfg(windows)]
fn enum_display_devices() -> Result<Vec<DeviceRecord>, String> {
    let output = run_pnputil_capture(&["/enum-devices", "/class", "Display"])?;
    parse_pnputil_devices(&output)
}

#[cfg(windows)]
fn run_pnputil(args: &[&str]) -> Result<(), String> {
    let status = Command::new("pnputil.exe")
        .args(args)
        .status()
        .map_err(|err| format!("Failed to run pnputil: {err}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("pnputil failed with exit code: {}", status))
    }
}

#[cfg(windows)]
fn run_pnputil_capture(args: &[&str]) -> Result<String, String> {
    let output = Command::new("pnputil.exe")
        .args(args)
        .output()
        .map_err(|err| format!("Failed to run pnputil: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "pnputil failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(windows)]
fn parse_pnputil_devices(output: &str) -> Result<Vec<DeviceRecord>, String> {
    let mut records = Vec::new();
    let mut current: Option<DeviceRecord> = None;

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            if let Some(record) = current.take() {
                records.push(record);
            }
            continue;
        }

        if let Some(value) = line.strip_prefix("Instance ID:") {
            if let Some(record) = current.take() {
                records.push(record);
            }
            current = Some(DeviceRecord {
                instance_id: value.trim().to_string(),
                description: String::new(),
                status: String::new(),
            });
        } else if let Some(value) = line.strip_prefix("Device Description:") {
            if let Some(record) = current.as_mut() {
                record.description = value.trim().to_string();
            }
        } else if let Some(value) = line.strip_prefix("Status:") {
            if let Some(record) = current.as_mut() {
                record.status = value.trim().to_string();
            }
        }
    }

    if let Some(record) = current.take() {
        records.push(record);
    }

    Ok(records)
}

#[cfg(windows)]
fn is_virtual_display_device(record: &DeviceRecord) -> bool {
    let instance = record.instance_id.to_ascii_lowercase();
    let description = record.description.to_ascii_lowercase();
    instance.contains("root\\mttvdd")
        || description.contains("virtual display")
        || description.contains("iddsample")
}

#[cfg(windows)]
fn find_device() -> Result<DeviceRecord, String> {
    let records = enum_display_devices()?;
    records
        .into_iter()
        .find(is_virtual_display_device)
        .ok_or_else(|| "Virtual display driver not found.".to_string())
}

#[cfg(windows)]
fn is_status_enabled(status: &str) -> bool {
    let lower = status.to_ascii_lowercase();
    lower.contains("started") || lower.contains("ok") || lower.contains("enabled")
}

#[cfg(windows)]
fn find_vendor_inf() -> Result<String, String> {
    let mut current = std::env::current_dir().map_err(|err| err.to_string())?;
    for _ in 0..6 {
        let candidate = current
            .join("_vendor")
            .join("virtual-display-driver")
            .join("Virtual Display Driver (HDR)")
            .join("MttVDD")
            .join("MttVDD.inf");
        if candidate.exists() {
            return candidate
                .to_str()
                .map(|value| value.to_string())
                .ok_or_else(|| "Invalid INF path.".to_string());
        }
        if !current.pop() {
            break;
        }
    }
    Err("MttVDD.inf not found under _vendor/virtual-display-driver.".to_string())
}

#[cfg(not(windows))]
pub fn status() -> Result<DriverManagerStatus, String> {
    Err("Virtual display driver management is only available on Windows.".to_string())
}

#[cfg(not(windows))]
pub fn enable() -> Result<(), String> {
    Err("Virtual display driver management is only available on Windows.".to_string())
}

#[cfg(not(windows))]
pub fn disable() -> Result<(), String> {
    Err("Virtual display driver management is only available on Windows.".to_string())
}

#[cfg(not(windows))]
pub fn toggle() -> Result<(), String> {
    Err("Virtual display driver management is only available on Windows.".to_string())
}

#[cfg(not(windows))]
pub fn install_from_vendor() -> Result<(), String> {
    Err("Virtual display driver management is only available on Windows.".to_string())
}

#[cfg(not(windows))]
pub fn uninstall() -> Result<(), String> {
    Err("Virtual display driver management is only available on Windows.".to_string())
}
