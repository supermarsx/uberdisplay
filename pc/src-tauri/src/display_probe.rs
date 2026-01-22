use crate::app_state::{DisplayInfo, DisplayMode};

#[cfg(windows)]
pub fn list_displays() -> Vec<DisplayInfo> {
    use windows::Win32::Graphics::Gdi::{
        EnumDisplayDevicesW, DISPLAY_DEVICEW, DISPLAY_DEVICE_ACTIVE, DISPLAY_DEVICE_PRIMARY_DEVICE,
    };
    use windows::core::PCWSTR;

    let mut displays = Vec::new();
    let mut device_index = 0;

    loop {
        let mut display_device = DISPLAY_DEVICEW::default();
        display_device.cb = std::mem::size_of::<DISPLAY_DEVICEW>() as u32;
        let ok = unsafe {
            EnumDisplayDevicesW(PCWSTR::null(), device_index, &mut display_device, 0)
        };
        if !ok.as_bool() {
            break;
        }

        let name = utf16_to_string(&display_device.DeviceString);
        let id = utf16_to_string(&display_device.DeviceName);
        let active = (display_device.StateFlags & DISPLAY_DEVICE_ACTIVE) != 0;
        let primary = (display_device.StateFlags & DISPLAY_DEVICE_PRIMARY_DEVICE) != 0;
        let is_virtual = looks_like_virtual_driver(&name);

        let (width, height, refresh_hz) = query_display_mode(&display_device.DeviceName);

        displays.push(DisplayInfo {
            id,
            name,
            active,
            primary,
            width,
            height,
            refresh_hz,
            is_virtual,
        });

        device_index += 1;
    }

    displays
}

#[cfg(not(windows))]
pub fn list_displays() -> Vec<DisplayInfo> {
    Vec::new()
}

#[cfg(windows)]
pub fn list_display_modes(display_id: &str) -> Vec<DisplayMode> {
    use std::collections::HashSet;
    use windows::Win32::Graphics::Gdi::{
        EnumDisplaySettingsExW, DEVMODEW, ENUM_DISPLAY_SETTINGS_FLAGS,
        ENUM_DISPLAY_SETTINGS_MODE,
    };
    use windows::core::PCWSTR;

    let name_wide = to_wide(display_id);
    let mut mode_index = 0u32;
    let mut seen = HashSet::new();
    let mut modes = Vec::new();

    loop {
        let mut devmode = DEVMODEW::default();
        devmode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;
        let ok = unsafe {
            EnumDisplaySettingsExW(
                PCWSTR::from_raw(name_wide.as_ptr()),
                ENUM_DISPLAY_SETTINGS_MODE(mode_index),
                &mut devmode,
                ENUM_DISPLAY_SETTINGS_FLAGS(0),
            )
        };
        if !ok.as_bool() {
            break;
        }

        let mode = DisplayMode {
            width: devmode.dmPelsWidth as i32,
            height: devmode.dmPelsHeight as i32,
            refresh_hz: devmode.dmDisplayFrequency as i32,
        };
        if seen.insert((mode.width, mode.height, mode.refresh_hz)) {
            modes.push(mode);
        }
        mode_index = mode_index.saturating_add(1);
    }

    modes
}

#[cfg(not(windows))]
pub fn list_display_modes(_display_id: &str) -> Vec<DisplayMode> {
    Vec::new()
}

#[cfg(windows)]
fn query_display_mode(device_name: &[u16]) -> (i32, i32, i32) {
    use windows::Win32::Graphics::Gdi::{
        EnumDisplaySettingsExW, DEVMODEW, ENUM_CURRENT_SETTINGS, ENUM_DISPLAY_SETTINGS_FLAGS,
    };
    use windows::core::PCWSTR;

    let mut devmode = DEVMODEW::default();
    devmode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;
    let ok = unsafe {
        EnumDisplaySettingsExW(
            PCWSTR::from_raw(device_name.as_ptr()),
            ENUM_CURRENT_SETTINGS,
            &mut devmode,
            ENUM_DISPLAY_SETTINGS_FLAGS(0),
        )
    };
    if ok.as_bool() {
        (
            devmode.dmPelsWidth as i32,
            devmode.dmPelsHeight as i32,
            devmode.dmDisplayFrequency as i32,
        )
    } else {
        (0, 0, 0)
    }
}

#[cfg(windows)]
fn looks_like_virtual_driver(device_name: &str) -> bool {
    let lower = device_name.to_ascii_lowercase();
    lower.contains("virtual")
        || lower.contains("indirect display")
        || lower.contains("uberdisplay")
}

#[cfg(windows)]
fn utf16_to_string(buffer: &[u16]) -> String {
    let len = buffer.iter().position(|&ch| ch == 0).unwrap_or(buffer.len());
    String::from_utf16_lossy(&buffer[..len])
}

#[cfg(windows)]
fn to_wide(value: &str) -> Vec<u16> {
    let mut wide: Vec<u16> = value.encode_utf16().collect();
    wide.push(0);
    wide
}
