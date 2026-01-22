use std::path::PathBuf;
use std::process::Command;

const SERVICE_NAME: &str = "UberDisplayVddService";

pub fn install_service() -> Result<(), String> {
    let exe_path = service_binary_path()?;
    let bin_arg = format!("binPath= {}", exe_path.display());
    let display_arg = "DisplayName= UberDisplay VDD Service";
    let status = Command::new("sc.exe")
        .args(["create", SERVICE_NAME, &bin_arg, "start= auto", display_arg])
        .status()
        .map_err(|err| format!("Failed to run sc.exe: {err}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("sc.exe create failed with {status}"))
    }
}

pub fn start_service() -> Result<(), String> {
    run_sc(&["start", SERVICE_NAME])
}

pub fn stop_service() -> Result<(), String> {
    run_sc(&["stop", SERVICE_NAME])
}

pub fn query_service() -> Result<String, String> {
    let output = Command::new("sc.exe")
        .args(["query", SERVICE_NAME])
        .output()
        .map_err(|err| format!("Failed to run sc.exe: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "sc.exe query failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn run_sc(args: &[&str]) -> Result<(), String> {
    let status = Command::new("sc.exe")
        .args(args)
        .status()
        .map_err(|err| format!("Failed to run sc.exe: {err}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("sc.exe failed with {status}"))
    }
}

fn service_binary_path() -> Result<PathBuf, String> {
    let current = std::env::current_exe().map_err(|err| err.to_string())?;
    if let Some(parent) = current.parent() {
        let candidate = parent.join("vdd_service.exe");
        if candidate.exists() {
            return Ok(candidate);
        }
    }
    Err("vdd_service.exe not found next to host binary.".to_string())
}

#[cfg(not(windows))]
pub fn install_service() -> Result<(), String> {
    Err("Service install is only available on Windows.".to_string())
}

#[cfg(not(windows))]
pub fn start_service() -> Result<(), String> {
    Err("Service start is only available on Windows.".to_string())
}

#[cfg(not(windows))]
pub fn stop_service() -> Result<(), String> {
    Err("Service stop is only available on Windows.".to_string())
}

#[cfg(not(windows))]
pub fn query_service() -> Result<String, String> {
    Err("Service query is only available on Windows.".to_string())
}
