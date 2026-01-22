#[cfg(target_os = "linux")]
use std::collections::BTreeMap;
#[cfg(target_os = "linux")]
use std::process::{Child, Command, Stdio};
#[cfg(target_os = "linux")]
use std::sync::{Mutex, OnceLock};

#[cfg(target_os = "linux")]
const DEFAULT_WIDTH: u32 = 2560;
#[cfg(target_os = "linux")]
const DEFAULT_HEIGHT: u32 = 1600;
#[cfg(target_os = "linux")]
const DEFAULT_DEPTH: u32 = 24;
#[cfg(target_os = "linux")]
const DEFAULT_BASE: u32 = 99;

#[cfg(target_os = "linux")]
struct LinuxDisplayState {
    displays: BTreeMap<u32, Child>,
    width: u32,
    height: u32,
    depth: u32,
    base_display: u32,
}

#[cfg(target_os = "linux")]
static STATE: OnceLock<Mutex<LinuxDisplayState>> = OnceLock::new();

#[cfg(target_os = "linux")]
fn state() -> &'static Mutex<LinuxDisplayState> {
    STATE.get_or_init(|| {
        Mutex::new(LinuxDisplayState {
            displays: BTreeMap::new(),
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            depth: DEFAULT_DEPTH,
            base_display: DEFAULT_BASE,
        })
    })
}

#[cfg(target_os = "linux")]
pub fn ensure_display_count(count: u32) -> Result<(), String> {
    ensure_xvfb_available()?;
    let mut guard = state().lock().map_err(|_| "Display state lock poisoned".to_string())?;
    while guard.displays.len() < count as usize {
        start_display_locked(&mut guard)?;
    }
    while guard.displays.len() > count as usize {
        stop_display_locked(&mut guard)?;
    }
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn current_count() -> u32 {
    state()
        .lock()
        .map(|guard| guard.displays.len() as u32)
        .unwrap_or(0)
}

#[cfg(target_os = "linux")]
pub fn set_config(base_display: u32, width: u32, height: u32, depth: u32) -> Result<(), String> {
    let mut guard = state().lock().map_err(|_| "Display state lock poisoned".to_string())?;
    guard.base_display = base_display;
    guard.width = width;
    guard.height = height;
    guard.depth = depth;
    if !guard.displays.is_empty() {
        let count = guard.displays.len() as u32;
        stop_all_locked(&mut guard)?;
        while guard.displays.len() < count as usize {
            start_display_locked(&mut guard)?;
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn ensure_xvfb_available() -> Result<(), String> {
    Command::new("Xvfb")
        .arg("-help")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|_| "Xvfb not found in PATH".to_string())
        .map(|_| ())
}

#[cfg(target_os = "linux")]
fn start_display_locked(state: &mut LinuxDisplayState) -> Result<(), String> {
    let display_num = next_display_number(state.base_display, &state.displays);
    let screen_spec = format!("{}x{}x{}", state.width, state.height, state.depth);
    let child = Command::new("Xvfb")
        .arg(format!(":{}", display_num))
        .arg("-screen")
        .arg("0")
        .arg(&screen_spec)
        .arg("-ac")
        .arg("-nolisten")
        .arg("tcp")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|err| format!("Failed to start Xvfb: {err}"))?;
    state.displays.insert(display_num, child);
    Ok(())
}

#[cfg(target_os = "linux")]
fn stop_display_locked(state: &mut LinuxDisplayState) -> Result<(), String> {
    let display_num = state
        .displays
        .keys()
        .next_back()
        .copied()
        .ok_or_else(|| "No Xvfb displays running".to_string())?;
    if let Some(mut child) = state.displays.remove(&display_num) {
        let _ = child.kill();
        let _ = child.wait();
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn stop_all_locked(state: &mut LinuxDisplayState) -> Result<(), String> {
    let keys: Vec<u32> = state.displays.keys().copied().collect();
    for display_num in keys.into_iter().rev() {
        if let Some(mut child) = state.displays.remove(&display_num) {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn next_display_number(base: u32, existing: &BTreeMap<u32, Child>) -> u32 {
    let mut candidate = base;
    for key in existing.keys() {
        if *key == candidate {
            candidate += 1;
        } else if *key > candidate {
            break;
        }
    }
    candidate
}

#[cfg(not(target_os = "linux"))]
pub fn ensure_display_count(_count: u32) -> Result<(), String> {
    Err("Linux virtual display management is only available on Linux.".to_string())
}

#[cfg(not(target_os = "linux"))]
pub fn current_count() -> u32 {
    0
}
