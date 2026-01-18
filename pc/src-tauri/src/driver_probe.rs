use crate::app_state::DriverStatus;

#[cfg(windows)]
pub fn probe_driver_status() -> DriverStatus {
    use windows::Win32::Graphics::Gdi::{EnumDisplayDevicesW, DISPLAY_DEVICEW, DISPLAY_DEVICE_ACTIVE};
    use windows::Win32::Foundation::PWSTR;

    let mut installed = false;
    let mut active = false;
    let mut device_index = 0;

    loop {
        let mut display_device = DISPLAY_DEVICEW::default();
        display_device.cb = std::mem::size_of::<DISPLAY_DEVICEW>() as u32;
        let ok = unsafe {
            EnumDisplayDevicesW(
                PWSTR::null(),
                device_index,
                &mut display_device,
                0,
            )
        };

        if !ok.as_bool() {
            break;
        }

        let name = utf16_to_string(&display_device.DeviceString);
        if looks_like_virtual_driver(&name) {
            installed = true;
            if display_device.StateFlags & DISPLAY_DEVICE_ACTIVE != 0 {
                active = true;
            }
        }

        device_index += 1;
    }

    DriverStatus { installed, active }
}

#[cfg(not(windows))]
pub fn probe_driver_status() -> DriverStatus {
    DriverStatus {
        installed: false,
        active: false,
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
