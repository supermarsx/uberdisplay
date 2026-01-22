use serde::Deserialize;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct DriverManagerStatus {
    pub installed: bool,
    pub status: String,
    #[serde(default)]
    pub details: Option<DriverManagerDetails>,
}

#[derive(Debug, Deserialize)]
pub struct DriverManagerDetails {
    #[serde(default)]
    pub friendlyName: Option<String>,
    #[serde(default)]
    pub instanceId: Option<String>,
    #[serde(default)]
    pub pnpDeviceID: Option<String>,
}

pub fn status() -> Result<DriverManagerStatus, String> {
    let output = run_script(&["-Action", "status", "-Json", "-Silent"])?;
    parse_status(&output)
}

pub fn action(action: &str) -> Result<(), String> {
    run_script(&["-Action", action, "-Silent"])?;
    Ok(())
}

fn run_script(args: &[&str]) -> Result<String, String> {
    let script = find_script_path()?;
    let output = Command::new("powershell.exe")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-NoProfile")
        .arg("-File")
        .arg(script)
        .args(args)
        .output()
        .map_err(|err| format!("Failed to run virtual driver manager: {err}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "Virtual driver manager failed: {}",
            stderr.trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn parse_status(output: &str) -> Result<DriverManagerStatus, String> {
    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Ok(status) = serde_json::from_str::<DriverManagerStatus>(trimmed) {
            return Ok(status);
        }
    }
    Err("Unable to parse driver status output.".to_string())
}

fn find_script_path() -> Result<PathBuf, String> {
    let mut current = std::env::current_dir().map_err(|err| err.to_string())?;
    for _ in 0..6 {
        let candidate = current.join("_vendor")
            .join("virtual-display-driver")
            .join("Community Scripts")
            .join("virtual-driver-manager.ps1");
        if candidate.exists() {
            return Ok(candidate);
        }
        if !current.pop() {
            break;
        }
    }
    Err("virtual-driver-manager.ps1 not found. Ensure the Virtual-Display-Driver repo is under _vendor/.".to_string())
}

#[cfg(not(windows))]
pub fn status() -> Result<DriverManagerStatus, String> {
    Err("Virtual driver manager is only available on Windows.".to_string())
}

#[cfg(not(windows))]
pub fn action(_action: &str) -> Result<(), String> {
    Err("Virtual driver manager is only available on Windows.".to_string())
}
