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
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, GetDC,
    ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
    SRCCOPY,
};
#[cfg(windows)]
use windows::Win32::Graphics::Dxgi::{
    IDXGIAdapter, IDXGIDevice, IDXGIOutput1, IDXGIOutputDuplication, IDXGIResource,
    DXGI_ERROR_ACCESS_LOST, DXGI_ERROR_WAIT_TIMEOUT, DXGI_OUTDUPL_FRAME_INFO,
};
#[cfg(windows)]
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SAMPLE_DESC};

#[cfg(windows)]
pub fn capture_nv12(width: i32, height: i32) -> Result<Vec<u8>, String> {
    let aligned_width = width.max(2) & !1;
    let aligned_height = height.max(2) & !1;
    let bgra = capture_bgra_dxgi(aligned_width, aligned_height)
        .or_else(|_| capture_bgra_gdi(aligned_width, aligned_height))?;
    Ok(bgra_to_nv12(&bgra, aligned_width, aligned_height))
}

#[cfg(not(windows))]
pub fn capture_nv12(_width: i32, _height: i32) -> Result<Vec<u8>, String> {
    Err("Capture not supported on this platform".to_string())
}

#[cfg(windows)]
fn capture_bgra_gdi(width: i32, height: i32) -> Result<Vec<u8>, String> {
    let hwnd = HWND(0);
    let screen_dc = unsafe { GetDC(hwnd) };
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
    let blit_ok = unsafe { BitBlt(mem_dc, 0, 0, width, height, screen_dc, 0, 0, SRCCOPY) };

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
        ReleaseDC(hwnd, screen_dc);
    }

    if blit_ok.is_ok() && rows > 0 {
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
struct DxgiCapture {
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    duplication: IDXGIOutputDuplication,
    staging: ID3D11Texture2D,
    width: i32,
    height: i32,
}

#[cfg(windows)]
static DXGI_CAPTURE: OnceLock<Mutex<Option<DxgiCapture>>> = OnceLock::new();

#[cfg(windows)]
fn capture_bgra_dxgi(width: i32, height: i32) -> Result<Vec<u8>, String> {
    let store = DXGI_CAPTURE.get_or_init(|| Mutex::new(None));
    let mut guard = store.lock().map_err(|_| "DXGI lock poisoned".to_string())?;

    let needs_init = match guard.as_ref() {
        Some(capture) => capture.width != width || capture.height != height,
        None => true,
    };
    if needs_init {
        *guard = Some(init_dxgi_capture(width, height)?);
    }

    let capture = guard.as_mut().ok_or_else(|| "DXGI capture not initialized".to_string())?;
    match acquire_dxgi_frame(capture) {
        Ok(frame) => Ok(frame),
        Err(err) => {
            *guard = None;
            Err(err)
        }
    }
}

#[cfg(windows)]
fn init_dxgi_capture(width: i32, height: i32) -> Result<DxgiCapture, String> {
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
            &mut device,
            None,
            &mut context,
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
    let output = unsafe { adapter.EnumOutputs(0) }
        .map_err(|err| format!("DXGI EnumOutputs failed: 0x{:08x}", err.code().0))?;
    let output1: IDXGIOutput1 = output
        .cast()
        .map_err(|err| format!("DXGI output cast failed: 0x{:08x}", err.code().0))?;
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
        BindFlags: D3D11_BIND_FLAG(0),
        CPUAccessFlags: D3D11_CPU_ACCESS_READ,
        MiscFlags: 0,
    };
    let staging = unsafe { device.CreateTexture2D(&desc, None) }
        .map_err(|err| format!("CreateTexture2D failed: 0x{:08x}", err.code().0))?;

    Ok(DxgiCapture {
        device,
        context,
        duplication,
        staging,
        width,
        height,
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
            return Err("DXGI capture timeout".to_string());
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
            .Map(&capture.staging, 0, D3D11_MAP_READ, 0, &mut mapped)
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
