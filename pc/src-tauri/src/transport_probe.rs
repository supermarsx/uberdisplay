use crate::app_state::TransportStatus;

const TCP_PORT: u16 = 1445;

pub fn probe_transport_status() -> TransportStatus {
    #[cfg(windows)]
    let tcp_probe = probe_tcp_table();
    #[cfg(not(windows))]
    let tcp_probe = TcpProbe {
        listening: probe_tcp_listener(),
        connections: 0,
    };

    TransportStatus {
        tcp_listening: tcp_probe.listening,
        tcp_connections: tcp_probe.connections,
        aoap_attached: probe_aoap_attached(),
    }
}

#[cfg(not(windows))]
fn probe_tcp_listener() -> bool {
    use std::net::{SocketAddr, TcpStream};
    use std::time::Duration;

    let addr = SocketAddr::from(([127, 0, 0, 1], TCP_PORT));
    TcpStream::connect_timeout(&addr, Duration::from_millis(200)).is_ok()
}

#[cfg(windows)]
struct TcpProbe {
    listening: bool,
    connections: u32,
}

#[cfg(windows)]
fn probe_tcp_table() -> TcpProbe {
    use std::mem::size_of;

    use windows::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, NO_ERROR};
    use windows::Win32::NetworkManagement::IpHelper::{
        GetExtendedTcpTable, MIB_TCP_STATE, MIB_TCP_STATE_ESTAB, MIB_TCP_STATE_LISTEN,
        MIB_TCPTABLE_OWNER_PID, TCP_TABLE_OWNER_PID_ALL,
    };
    use windows::Win32::Networking::WinSock::AF_INET;

    let mut size: u32 = 0;
    let mut result = unsafe {
        GetExtendedTcpTable(
            None,
            &mut size,
            false,
            AF_INET.0 as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        )
    };

    if result != ERROR_INSUFFICIENT_BUFFER.0 {
        return TcpProbe {
            listening: false,
            connections: 0,
        };
    }

    let mut buffer = vec![0u8; size as usize];
    result = unsafe {
        GetExtendedTcpTable(
            Some(buffer.as_mut_ptr() as *mut _),
            &mut size,
            false,
            AF_INET.0 as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        )
    };

    if result != NO_ERROR.0 {
        return TcpProbe {
            listening: false,
            connections: 0,
        };
    }

    if buffer.len() < size_of::<MIB_TCPTABLE_OWNER_PID>() {
        return TcpProbe {
            listening: false,
            connections: 0,
        };
    }

    let table = buffer.as_ptr() as *const MIB_TCPTABLE_OWNER_PID;
    let entries = unsafe { (*table).dwNumEntries as usize };
    let rows = unsafe { std::slice::from_raw_parts((*table).table.as_ptr(), entries) };

    let mut listening = false;
    let mut connections = 0u32;

    for row in rows {
        let port = u16::from_be(row.dwLocalPort as u16);
        if port != TCP_PORT {
            continue;
        }
        if MIB_TCP_STATE(row.dwState as i32) == MIB_TCP_STATE_LISTEN {
            listening = true;
        } else if MIB_TCP_STATE(row.dwState as i32) == MIB_TCP_STATE_ESTAB {
            connections += 1;
        }
    }

    TcpProbe {
        listening,
        connections,
    }
}

#[cfg(windows)]
fn probe_aoap_attached() -> bool {
    use std::mem::size_of;

    use windows::Win32::Devices::DeviceAndDriverInstallation::{
        SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo, SetupDiGetClassDevsW, DIGCF_ALLCLASSES,
        DIGCF_PRESENT, SP_DEVINFO_DATA, SPDRP_DEVICEDESC, SPDRP_FRIENDLYNAME,
    };

    let device_info_set =
        unsafe { SetupDiGetClassDevsW(None, None, None, DIGCF_PRESENT | DIGCF_ALLCLASSES) };
    let device_info_set = match device_info_set {
        Ok(handle) => handle,
        Err(_) => return false,
    };
    if device_info_set.is_invalid() {
        return false;
    }

    let mut index = 0;
    let mut attached = false;

    loop {
        let mut device_info = SP_DEVINFO_DATA {
            cbSize: size_of::<SP_DEVINFO_DATA>() as u32,
            ..Default::default()
        };
        let success =
            unsafe { SetupDiEnumDeviceInfo(device_info_set, index, &mut device_info) }.is_ok();
        if !success {
            break;
        }

        let name = get_device_string(device_info_set, &device_info, SPDRP_FRIENDLYNAME)
            .or_else(|| get_device_string(device_info_set, &device_info, SPDRP_DEVICEDESC));

        if let Some(name) = name {
            let lower = name.to_ascii_lowercase();
            if lower.contains("android")
                || lower.contains("adb")
                || lower.contains("aoap")
                || (lower.contains("accessory") && lower.contains("usb"))
            {
                attached = true;
                break;
            }
        }

        index += 1;
    }

    let _ = unsafe { SetupDiDestroyDeviceInfoList(device_info_set) };

    attached
}

#[cfg(not(windows))]
fn probe_aoap_attached() -> bool {
    false
}

#[cfg(windows)]
fn get_device_string(
    device_info_set: windows::Win32::Devices::DeviceAndDriverInstallation::HDEVINFO,
    device_info: &windows::Win32::Devices::DeviceAndDriverInstallation::SP_DEVINFO_DATA,
    property: windows::Win32::Devices::DeviceAndDriverInstallation::SETUP_DI_REGISTRY_PROPERTY,
) -> Option<String> {
    use windows::Win32::Devices::DeviceAndDriverInstallation::SetupDiGetDeviceRegistryPropertyW;
    use windows::Win32::Foundation::{GetLastError, ERROR_INSUFFICIENT_BUFFER};

    let mut data_type = 0u32;
    let mut required_size = 0u32;
    let mut buffer = vec![0u8; 256];

    let mut success = unsafe {
        SetupDiGetDeviceRegistryPropertyW(
            device_info_set,
            device_info,
            property,
            Some(&mut data_type),
            Some(buffer.as_mut_slice()),
            Some(&mut required_size),
        )
    }
    .is_ok();

    if !success {
        let err = unsafe { GetLastError() };
        if err != ERROR_INSUFFICIENT_BUFFER || required_size == 0 {
            return None;
        }
        buffer = vec![0u8; required_size as usize];
        success = unsafe {
            SetupDiGetDeviceRegistryPropertyW(
                device_info_set,
                device_info,
                property,
                Some(&mut data_type),
                Some(buffer.as_mut_slice()),
                Some(&mut required_size),
            )
        }
        .is_ok();
        if !success {
            return None;
        }
    }

    let utf16: Vec<u16> = buffer
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();
    Some(utf16_to_string(&utf16))
}

#[cfg(windows)]
fn utf16_to_string(buffer: &[u16]) -> String {
    let len = buffer.iter().position(|&ch| ch == 0).unwrap_or(buffer.len());
    String::from_utf16_lossy(&buffer[..len])
}
