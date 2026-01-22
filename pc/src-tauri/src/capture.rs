#[cfg(windows)]
use std::mem::size_of;

#[cfg(windows)]
use windows::Win32::Foundation::HWND;
#[cfg(windows)]
use windows::Win32::Graphics::Gdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, GetDC,
    ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
    SRCCOPY,
};

#[cfg(windows)]
pub fn capture_nv12(width: i32, height: i32) -> Result<Vec<u8>, String> {
    let aligned_width = width.max(2) & !1;
    let aligned_height = height.max(2) & !1;
    let bgra = capture_bgra(aligned_width, aligned_height)?;
    Ok(bgra_to_nv12(&bgra, aligned_width, aligned_height))
}

#[cfg(not(windows))]
pub fn capture_nv12(_width: i32, _height: i32) -> Result<Vec<u8>, String> {
    Err("Capture not supported on this platform".to_string())
}

#[cfg(windows)]
fn capture_bgra(width: i32, height: i32) -> Result<Vec<u8>, String> {
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
