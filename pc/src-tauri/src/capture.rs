#[cfg(windows)]
use std::mem::size_of;
#[cfg(windows)]
use std::sync::{Mutex, OnceLock};

#[cfg(windows)]
use windows::Win32::Foundation::HWND;
#[cfg(windows)]
use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_HARDWARE;
#[cfg(windows)]
use windows::Win32::Graphics::Direct3D11::{
    D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext, ID3D11Texture2D,
    D3D11_BIND_FLAG, D3D11_CPU_ACCESS_READ, D3D11_CREATE_DEVICE_BGRA_SUPPORT,
    D3D11_MAP_READ, D3D11_MAPPED_SUBRESOURCE, D3D11_SDK_VERSION, D3D11_TEXTURE2D_DESC,
    D3D11_USAGE_STAGING,
};
#[cfg(windows)]
use windows::Win32::Graphics::Gdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, CreateDCW, DeleteDC, DeleteObject,
    EnumDisplaySettingsExW, GetDIBits, GetDC, ReleaseDC, SelectObject, SetStretchBltMode,
    StretchBlt, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HALFTONE, SRCCOPY,
};
#[cfg(windows)]
use windows::Win32::Graphics::Dxgi::{
    IDXGIAdapter, IDXGIDevice, IDXGIOutput, IDXGIOutput1, IDXGIOutputDuplication, IDXGIResource,
    DXGI_ERROR_ACCESS_LOST, DXGI_ERROR_WAIT_TIMEOUT, DXGI_OUTDUPL_FRAME_INFO, DXGI_OUTPUT_DESC,
};
#[cfg(windows)]
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SAMPLE_DESC};
#[cfg(windows)]
use windows::core::Interface;
#[cfg(windows)]
use windows::core::PCWSTR;
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

#[cfg(windows)]
pub fn capture_nv12(width: i32, height: i32) -> Result<Vec<u8>, String> {
    let aligned_width = width.max(2) & !1;
    let aligned_height = height.max(2) & !1;
    let bgra = match capture_bgra_dxgi(aligned_width, aligned_height, None) {
        Ok(frame) => frame,
        Err(_) => capture_bgra_gdi(aligned_width, aligned_height, None)?,
    };
    Ok(bgra_to_nv12(&bgra, aligned_width, aligned_height))
}

#[cfg(windows)]
pub fn capture_nv12_with_target(
    width: i32,
    height: i32,
    target_id: Option<&str>,
) -> Result<Vec<u8>, String> {
    let aligned_width = width.max(2) & !1;
    let aligned_height = height.max(2) & !1;
    let bgra = match capture_bgra_dxgi(aligned_width, aligned_height, target_id) {
        Ok(frame) => frame,
        Err(_) => capture_bgra_gdi(aligned_width, aligned_height, target_id)?,
    };
    Ok(bgra_to_nv12(&bgra, aligned_width, aligned_height))
}

#[cfg(not(windows))]
pub fn capture_nv12(_width: i32, _height: i32) -> Result<Vec<u8>, String> {
    Err("Capture not supported on this platform".to_string())
}

#[cfg(windows)]
fn capture_bgra_gdi(width: i32, height: i32, target_id: Option<&str>) -> Result<Vec<u8>, String> {
    let hwnd = HWND(0);
    let screen_dc = if let Some(name) = target_id {
        let driver_wide = to_wide("DISPLAY");
        let name_wide = to_wide(name);
        unsafe {
            CreateDCW(
                PCWSTR::from_raw(driver_wide.as_ptr()),
                PCWSTR::from_raw(name_wide.as_ptr()),
                PCWSTR::null(),
                None,
            )
        }
    } else {
        unsafe { GetDC(hwnd) }
    };
    if screen_dc.0 == 0 {
        return Err("GetDC failed".to_string());
    }
    let mem_dc = unsafe { CreateCompatibleDC(screen_dc) };
    if mem_dc.0 == 0 {
        unsafe {
            ReleaseDC(hwnd, screen_dc);
        }
        return Err("CreateCompatibleDC failed".to_string());
    }
    let bitmap = unsafe { CreateCompatibleBitmap(screen_dc, width, height) };
    if bitmap.0 == 0 {
        unsafe {
            DeleteDC(mem_dc);
            ReleaseDC(hwnd, screen_dc);
        }
        return Err("CreateCompatibleBitmap failed".to_string());
    }

    let old = unsafe { SelectObject(mem_dc, bitmap) };
    let (screen_width, screen_height) = if let Some(name) = target_id {
        query_display_dimensions(name)
    } else {
        (unsafe { GetSystemMetrics(SM_CXSCREEN) }, unsafe { GetSystemMetrics(SM_CYSCREEN) })
    };
    let scale_mode = if screen_width == width && screen_height == height {
        "1:1"
    } else {
        "Scaled"
    };
    let blit_ok = if screen_width == width && screen_height == height {
        unsafe { BitBlt(mem_dc, 0, 0, width, height, screen_dc, 0, 0, SRCCOPY) }
            .is_ok()
    } else {
        unsafe {
            let _ = SetStretchBltMode(mem_dc, HALFTONE);
            StretchBlt(
                mem_dc,
                0,
                0,
                width,
                height,
                screen_dc,
                0,
                0,
                screen_width,
                screen_height,
                SRCCOPY,
            )
        }
        .as_bool()
    };

    let mut info = BITMAPINFO::default();
    info.bmiHeader = BITMAPINFOHEADER {
        biSize: size_of::<BITMAPINFOHEADER>() as u32,
        biWidth: width,
        biHeight: -height,
        biPlanes: 1,
        biBitCount: 32,
        biCompression: BI_RGB.0,
        biSizeImage: 0,
        biXPelsPerMeter: 0,
        biYPelsPerMeter: 0,
        biClrUsed: 0,
        biClrImportant: 0,
    };

    let buffer_len = (width as usize) * (height as usize) * 4;
    let mut buffer = vec![0u8; buffer_len];
    let rows = unsafe {
        GetDIBits(
            mem_dc,
            bitmap,
            0,
            height as u32,
            Some(buffer.as_mut_ptr() as *mut _),
            &mut info,
            DIB_RGB_COLORS,
        )
    };

    unsafe {
        SelectObject(mem_dc, old);
        DeleteObject(bitmap);
        DeleteDC(mem_dc);
        if target_id.is_some() {
            DeleteDC(screen_dc);
        } else {
            ReleaseDC(hwnd, screen_dc);
        }
    }

    update_capture_info("GDI", scale_mode);

    if blit_ok && rows > 0 {
        Ok(buffer)
    } else {
        Err("GetDIBits failed".to_string())
    }
}

#[cfg(windows)]
fn bgra_to_nv12(bgra: &[u8], width: i32, height: i32) -> Vec<u8> {
    let w = width as usize;
    let h = height as usize;
    let mut y_plane = vec![0u8; w * h];
    let mut uv_plane = vec![0u8; w * h / 2];

    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) * 4;
            let b = bgra[idx] as i32;
            let g = bgra[idx + 1] as i32;
            let r = bgra[idx + 2] as i32;
            let y_val = clamp_u8(((66 * r + 129 * g + 25 * b + 128) >> 8) + 16);
            y_plane[y * w + x] = y_val;
        }
    }

    for y in (0..h).step_by(2) {
        for x in (0..w).step_by(2) {
            let mut u_sum = 0i32;
            let mut v_sum = 0i32;
            for dy in 0..2 {
                for dx in 0..2 {
                    let idx = ((y + dy) * w + (x + dx)) * 4;
                    let b = bgra[idx] as i32;
                    let g = bgra[idx + 1] as i32;
                    let r = bgra[idx + 2] as i32;
                    let u_val = ((-38 * r - 74 * g + 112 * b + 128) >> 8) + 128;
                    let v_val = ((112 * r - 94 * g - 18 * b + 128) >> 8) + 128;
                    u_sum += u_val;
                    v_sum += v_val;
                }
            }
            let uv_index = (y / 2) * w + x;
            uv_plane[uv_index] = clamp_u8(u_sum / 4);
            uv_plane[uv_index + 1] = clamp_u8(v_sum / 4);
        }
    }

    let mut output = Vec::with_capacity(y_plane.len() + uv_plane.len());
    output.extend_from_slice(&y_plane);
    output.extend_from_slice(&uv_plane);
    output
}

#[cfg(windows)]
fn clamp_u8(value: i32) -> u8 {
    value.clamp(0, 255) as u8
}

#[cfg(windows)]
fn select_output(adapter: &IDXGIAdapter, target_id: Option<&str>) -> Result<IDXGIOutput1, String> {
    let mut index = 0u32;
    loop {
        let output: IDXGIOutput = match unsafe { adapter.EnumOutputs(index) } {
            Ok(output) => output,
            Err(_) => break,
        };
        let output1: IDXGIOutput1 = output
            .cast()
            .map_err(|err| format!("DXGI output cast failed: 0x{:08x}", err.code().0))?;
        let mut desc = DXGI_OUTPUT_DESC::default();
        unsafe {
            output
                .GetDesc(&mut desc)
                .map_err(|err| format!("DXGI GetDesc failed: 0x{:08x}", err.code().0))?;
        }
        let name = utf16_to_string(&desc.DeviceName);
        if target_id.map(|target| target.eq_ignore_ascii_case(&name)).unwrap_or(true) {
            return Ok(output1);
        }
        index = index.saturating_add(1);
    }
    Err("DXGI output not found".to_string())
}

#[cfg(windows)]
fn query_display_dimensions(display_id: &str) -> (i32, i32) {
    use windows::Win32::Graphics::Gdi::{
        EnumDisplaySettingsExW, DEVMODEW, ENUM_CURRENT_SETTINGS, ENUM_DISPLAY_SETTINGS_FLAGS,
    };
    let name_wide = to_wide(display_id);
    let mut devmode = DEVMODEW::default();
    devmode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;
    let ok = unsafe {
        EnumDisplaySettingsExW(
            PCWSTR::from_raw(name_wide.as_ptr()),
            ENUM_CURRENT_SETTINGS,
            &mut devmode,
            ENUM_DISPLAY_SETTINGS_FLAGS(0),
        )
    };
    if ok.as_bool() {
        (devmode.dmPelsWidth as i32, devmode.dmPelsHeight as i32)
    } else {
        (unsafe { GetSystemMetrics(SM_CXSCREEN) }, unsafe { GetSystemMetrics(SM_CYSCREEN) })
    }
}

#[cfg(windows)]
fn to_wide(value: &str) -> Vec<u16> {
    let mut wide: Vec<u16> = value.encode_utf16().collect();
    wide.push(0);
    wide
}

#[cfg(windows)]
fn utf16_to_string(buffer: &[u16]) -> String {
    let len = buffer.iter().position(|&ch| ch == 0).unwrap_or(buffer.len());
    String::from_utf16_lossy(&buffer[..len])
}

#[cfg(windows)]
struct DxgiCapture {
    #[allow(dead_code)]
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    duplication: IDXGIOutputDuplication,
    staging: ID3D11Texture2D,
    width: i32,
    height: i32,
    timeouts: u32,
    access_lost: u32,
    frame_failures: u32,
    last_frame_bytes: u32,
    capture_path: String,
    capture_scale: String,
    target_id: Option<String>,
}

#[cfg(windows)]
pub struct DxgiFrame {
    duplication: IDXGIOutputDuplication,
    pub texture: ID3D11Texture2D,
}

#[cfg(windows)]
impl Drop for DxgiFrame {
    fn drop(&mut self) {
        unsafe {
            let _ = self.duplication.ReleaseFrame();
        }
    }
}

#[cfg(windows)]
static DXGI_CAPTURE: OnceLock<Mutex<Option<DxgiCapture>>> = OnceLock::new();

#[cfg(windows)]
fn capture_bgra_dxgi(
    width: i32,
    height: i32,
    target_id: Option<&str>,
) -> Result<Vec<u8>, String> {
    let store = DXGI_CAPTURE.get_or_init(|| Mutex::new(None));
    let mut guard = store.lock().map_err(|_| "DXGI lock poisoned".to_string())?;

    let needs_init = match guard.as_ref() {
        Some(capture) => {
            capture.width != width
                || capture.height != height
                || capture.target_id.as_deref() != target_id
        }
        None => true,
    };
    if needs_init {
        *guard = Some(init_dxgi_capture(width, height, target_id)?);
    }

    let capture = guard.as_mut().ok_or_else(|| "DXGI capture not initialized".to_string())?;
    let frame = match acquire_dxgi_frame(capture) {
        Ok(frame) => frame,
        Err(err) => {
            capture.frame_failures = capture.frame_failures.saturating_add(1);
            *guard = None;
            return Err(err);
        }
    };
    capture.last_frame_bytes = frame.len() as u32;
    capture.capture_path = "DXGI".to_string();
    capture.capture_scale = "1:1".to_string();
    Ok(frame)
}

#[cfg(windows)]
pub fn capture_dxgi_surface(
    width: i32,
    height: i32,
    target_id: Option<&str>,
) -> Result<DxgiFrame, String> {
    let store = DXGI_CAPTURE.get_or_init(|| Mutex::new(None));
    let mut guard = store.lock().map_err(|_| "DXGI lock poisoned".to_string())?;

    let needs_init = match guard.as_ref() {
        Some(capture) => {
            capture.width != width
                || capture.height != height
                || capture.target_id.as_deref() != target_id
        }
        None => true,
    };
    if needs_init {
        *guard = Some(init_dxgi_capture(width, height, target_id)?);
    }

    let capture = guard.as_mut().ok_or_else(|| "DXGI capture not initialized".to_string())?;
    match acquire_dxgi_surface(capture) {
        Ok(frame) => Ok(frame),
        Err(err) => {
            capture.frame_failures = capture.frame_failures.saturating_add(1);
            *guard = None;
            Err(err)
        }
    }
}

#[cfg(windows)]
fn init_dxgi_capture(
    width: i32,
    height: i32,
    target_id: Option<&str>,
) -> Result<DxgiCapture, String> {
    let mut device: Option<ID3D11Device> = None;
    let mut context: Option<ID3D11DeviceContext> = None;
    unsafe {
        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            None,
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            None,
            D3D11_SDK_VERSION,
            Some(&mut device),
            None,
            Some(&mut context),
        )
        .map_err(|err| format!("D3D11CreateDevice failed: 0x{:08x}", err.code().0))?;
    }
    let device = device.ok_or_else(|| "D3D11 device unavailable".to_string())?;
    let context = context.ok_or_else(|| "D3D11 context unavailable".to_string())?;

    let dxgi_device: IDXGIDevice = device
        .cast()
        .map_err(|err| format!("DXGI device cast failed: 0x{:08x}", err.code().0))?;
    let adapter: IDXGIAdapter = unsafe { dxgi_device.GetAdapter() }
        .map_err(|err| format!("DXGI GetAdapter failed: 0x{:08x}", err.code().0))?;
    let output1 = select_output(&adapter, target_id)?;
    let duplication = unsafe { output1.DuplicateOutput(&device) }
        .map_err(|err| format!("DXGI DuplicateOutput failed: 0x{:08x}", err.code().0))?;

    let desc = D3D11_TEXTURE2D_DESC {
        Width: width as u32,
        Height: height as u32,
        MipLevels: 1,
        ArraySize: 1,
        Format: DXGI_FORMAT_B8G8R8A8_UNORM,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Usage: D3D11_USAGE_STAGING,
        BindFlags: D3D11_BIND_FLAG(0).0 as u32,
        CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
        MiscFlags: 0,
    };
    let mut staging: Option<ID3D11Texture2D> = None;
    unsafe {
        device
            .CreateTexture2D(&desc, None, Some(&mut staging))
            .map_err(|err| format!("CreateTexture2D failed: 0x{:08x}", err.code().0))?;
    }
    let staging = staging.ok_or_else(|| "Staging texture unavailable".to_string())?;

    Ok(DxgiCapture {
        device,
        context,
        duplication,
        staging,
        width,
        height,
        timeouts: 0,
        access_lost: 0,
        frame_failures: 0,
        last_frame_bytes: 0,
        capture_path: "DXGI".to_string(),
        capture_scale: "1:1".to_string(),
        target_id: target_id.map(|value| value.to_string()),
    })
}

#[cfg(windows)]
fn acquire_dxgi_frame(capture: &mut DxgiCapture) -> Result<Vec<u8>, String> {
    let mut frame_info = DXGI_OUTDUPL_FRAME_INFO::default();
    let mut resource: Option<IDXGIResource> = None;
    let result = unsafe { capture.duplication.AcquireNextFrame(16, &mut frame_info, &mut resource) };
    if let Err(err) = result {
        let code = err.code();
        if code == DXGI_ERROR_WAIT_TIMEOUT || code == DXGI_ERROR_ACCESS_LOST {
            if code == DXGI_ERROR_WAIT_TIMEOUT {
                capture.timeouts = capture.timeouts.saturating_add(1);
                return Err("DXGI capture timeout".to_string());
            }
            capture.access_lost = capture.access_lost.saturating_add(1);
            return Err("DXGI capture access lost".to_string());
        }
        return Err(format!("DXGI AcquireNextFrame failed: 0x{:08x}", code.0));
    }

    let resource = resource.ok_or_else(|| "DXGI resource missing".to_string())?;
    let texture: ID3D11Texture2D = resource
        .cast()
        .map_err(|err| format!("DXGI resource cast failed: 0x{:08x}", err.code().0))?;
    unsafe {
        capture
            .context
            .CopyResource(&capture.staging, &texture);
    }

    let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
    unsafe {
        capture
            .context
            .Map(&capture.staging, 0, D3D11_MAP_READ, 0, Some(&mut mapped))
            .map_err(|err| format!("DXGI Map failed: 0x{:08x}", err.code().0))?;
    }

    let width = capture.width as usize;
    let height = capture.height as usize;
    let row_pitch = mapped.RowPitch as usize;
    let src = mapped.pData as *const u8;
    let mut buffer = vec![0u8; width * height * 4];
    for y in 0..height {
        let src_row = unsafe { src.add(y * row_pitch) };
        let dst_row = &mut buffer[y * width * 4..(y + 1) * width * 4];
        unsafe {
            std::ptr::copy_nonoverlapping(src_row, dst_row.as_mut_ptr(), width * 4);
        }
    }
    unsafe {
        capture.context.Unmap(&capture.staging, 0);
        let _ = capture.duplication.ReleaseFrame();
    }
    Ok(buffer)
}

#[cfg(windows)]
fn acquire_dxgi_surface(capture: &mut DxgiCapture) -> Result<DxgiFrame, String> {
    let mut frame_info = DXGI_OUTDUPL_FRAME_INFO::default();
    let mut resource: Option<IDXGIResource> = None;
    let result = unsafe { capture.duplication.AcquireNextFrame(16, &mut frame_info, &mut resource) };
    if let Err(err) = result {
        let code = err.code();
        if code == DXGI_ERROR_WAIT_TIMEOUT || code == DXGI_ERROR_ACCESS_LOST {
            if code == DXGI_ERROR_WAIT_TIMEOUT {
                capture.timeouts = capture.timeouts.saturating_add(1);
                return Err("DXGI capture timeout".to_string());
            }
            capture.access_lost = capture.access_lost.saturating_add(1);
            return Err("DXGI capture access lost".to_string());
        }
        return Err(format!("DXGI AcquireNextFrame failed: 0x{:08x}", code.0));
    }

    let resource = resource.ok_or_else(|| "DXGI resource missing".to_string())?;
    let texture: ID3D11Texture2D = resource
        .cast()
        .map_err(|err| format!("DXGI resource cast failed: 0x{:08x}", err.code().0))?;

    Ok(DxgiFrame {
        duplication: capture.duplication.clone(),
        texture,
    })
}

#[cfg(windows)]
pub fn dxgi_stats_snapshot() -> Option<(u32, u32, u32, u32)> {
    let store = DXGI_CAPTURE.get_or_init(|| Mutex::new(None));
    let guard = store.lock().ok()?;
    let capture = guard.as_ref()?;
    Some((
        capture.timeouts,
        capture.access_lost,
        capture.frame_failures,
        capture.last_frame_bytes,
    ))
}

#[cfg(not(windows))]
pub fn dxgi_stats_snapshot() -> Option<(u32, u32, u32, u32)> {
    None
}

#[cfg(windows)]
pub fn capture_info_snapshot() -> Option<(String, String)> {
    let store = DXGI_CAPTURE.get_or_init(|| Mutex::new(None));
    let guard = store.lock().ok()?;
    if let Some(capture) = guard.as_ref() {
        return Some((capture.capture_path.clone(), capture.capture_scale.clone()));
    }
    Some(("GDI".to_string(), "Scaled".to_string()))
}

#[cfg(not(windows))]
pub fn capture_info_snapshot() -> Option<(String, String)> {
    None
}

#[cfg(windows)]
fn update_capture_info(path: &str, scale: &str) {
    let store = DXGI_CAPTURE.get_or_init(|| Mutex::new(None));
    if let Ok(mut guard) = store.lock() {
        if let Some(capture) = guard.as_mut() {
            capture.capture_path = path.to_string();
            capture.capture_scale = scale.to_string();
        }
    }
}
