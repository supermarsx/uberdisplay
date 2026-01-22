use crate::codec::CodecId;
#[cfg(windows)]
use crate::capture;

#[cfg(windows)]
use windows::core::GUID;
#[cfg(windows)]
use windows::Win32::Media::MediaFoundation::{
    IMFActivate, IMFMediaBuffer, IMFMediaType, IMFTransform, MFCreateMediaType, MFCreateMemoryBuffer,
    MFCreateSample, MFCreateDXGISurfaceBuffer, MFShutdown, MFStartup, MFTEnumEx,
    MFT_OUTPUT_DATA_BUFFER, MFT_ENUM_FLAG_LOCALMFT, MFT_ENUM_FLAG_SYNCMFT,
    MFT_MESSAGE_COMMAND_FLUSH, MFT_MESSAGE_COMMAND_DRAIN, MFT_MESSAGE_NOTIFY_BEGIN_STREAMING,
    MFT_MESSAGE_NOTIFY_START_OF_STREAM, MFT_REGISTER_TYPE_INFO, MFT_CATEGORY_VIDEO_ENCODER,
    MFMediaType_Video, MFVideoFormat_H264, MFVideoFormat_HEVC, MFVideoFormat_NV12, MFVideoFormat_ARGB32,
    MF_E_TRANSFORM_NEED_MORE_INPUT, MF_MT_AVG_BITRATE, MF_MT_FRAME_RATE, MF_MT_FRAME_SIZE,
    MF_MT_INTERLACE_MODE, MF_MT_MAJOR_TYPE, MF_MT_PIXEL_ASPECT_RATIO, MF_MT_SUBTYPE,
    MFVideoInterlace_Progressive, MF_VERSION,
};
#[cfg(windows)]
use windows::Win32::System::Com::{
    CoInitializeEx, CoTaskMemFree, CoUninitialize, COINIT_MULTITHREADED,
};
#[cfg(windows)]
use windows::core::Interface;
#[cfg(windows)]
use windows::Win32::Graphics::Direct3D11::ID3D11Texture2D;
#[cfg(windows)]
use std::mem::ManuallyDrop;

pub struct MfEncoder {
    pub codec_id: CodecId,
    pub width: i32,
    pub height: i32,
    pub bitrate_kbps: u32,
    pub fps: u32,
    pub keyframe_interval: u32,
    frame_index: u64,
    #[cfg(windows)]
    com_initialized: bool,
    #[cfg(windows)]
    mf_started: bool,
    #[cfg(windows)]
    encoder_available: bool,
    #[cfg(windows)]
    transform: Option<IMFTransform>,
    #[cfg(windows)]
    output_buffer_len: u32,
    #[cfg(windows)]
    last_error: Option<String>,
    #[cfg(windows)]
    use_dxgi_surface: bool,
}

impl MfEncoder {
    pub fn new(
        codec_id: CodecId,
        width: i32,
        height: i32,
        bitrate_kbps: u32,
        fps: u32,
        keyframe_interval: u32,
    ) -> Result<Self, String> {
        let aligned_width = (width.max(2)) & !1;
        let aligned_height = (height.max(2)) & !1;
        match codec_id {
            CodecId::H264 | CodecId::H265 => {
                #[cfg(windows)]
                let init = init_media_foundation(
                    codec_id,
                    aligned_width,
                    aligned_height,
                    bitrate_kbps,
                    fps,
                )?;
                Ok(Self {
                    codec_id,
                    width: aligned_width,
                    height: aligned_height,
                    bitrate_kbps,
                    fps,
                    keyframe_interval,
                    frame_index: 0,
                    #[cfg(windows)]
                    com_initialized: init.com_initialized,
                    #[cfg(windows)]
                    mf_started: init.mf_started,
                    #[cfg(windows)]
                    encoder_available: init.encoder_available,
                    #[cfg(windows)]
                    transform: init.transform,
                    #[cfg(windows)]
                    output_buffer_len: init.output_buffer_len,
                    #[cfg(windows)]
                    last_error: None,
                    #[cfg(windows)]
                    use_dxgi_surface: init.use_dxgi_surface,
                })
            }
            _ => Err("Media Foundation encoder supports H.264/H.265 only".to_string()),
        }
    }

    pub fn encode_frame(&mut self) -> (Vec<u8>, Option<u64>) {
        #[cfg(windows)]
        if let Some((payload, timestamp)) = self.encode_mf_frame() {
            if !payload.is_empty() {
                return (payload, Some(timestamp));
            }
        }

        // Placeholder until Media Foundation pipeline is implemented.
        // Size approximates bitrate/fps to keep pacing behavior realistic.
        let fps = self.fps.max(1);
        let bitrate_bytes = ((self.bitrate_kbps as u64 * 1000) / 8 / fps as u64)
            .clamp(128, 512 * 1024);
        let pixel_area = (self.width.max(1) as u64) * (self.height.max(1) as u64);
        let resolution_bytes = (pixel_area / 80).clamp(256, 512 * 1024);
        let mut bytes_per_frame = bitrate_bytes.max(resolution_bytes);
        #[cfg(windows)]
        if !self.encoder_available {
            bytes_per_frame = (bytes_per_frame / 2).max(128);
        }
        if self.codec_id == CodecId::H265 {
            bytes_per_frame = bytes_per_frame.saturating_mul(9) / 10;
        }
        self.frame_index = self.frame_index.wrapping_add(1);
        if self.keyframe_interval > 0
            && self.frame_index % self.keyframe_interval as u64 == 0
        {
            bytes_per_frame = bytes_per_frame.saturating_mul(2).min(768 * 1024);
        }
        let timestamp = estimate_timestamp_100ns(self.frame_index, self.fps);
        (vec![0u8; bytes_per_frame as usize], Some(timestamp))
    }
}

#[cfg(windows)]
impl Drop for MfEncoder {
    fn drop(&mut self) {
    if self.mf_started {
        unsafe {
            let _ = MFShutdown();
        }
    }
        if self.com_initialized {
            unsafe {
                CoUninitialize();
            }
        }
    }
}

#[cfg(windows)]
struct MfInit {
    com_initialized: bool,
    mf_started: bool,
    encoder_available: bool,
    transform: Option<IMFTransform>,
    output_buffer_len: u32,
    use_dxgi_surface: bool,
}

#[cfg(windows)]
fn init_media_foundation(
    codec_id: CodecId,
    width: i32,
    height: i32,
    bitrate_kbps: u32,
    fps: u32,
) -> Result<MfInit, String> {
    let com_result = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
    let com_initialized = com_result.is_ok();
    if !com_initialized {
        return Err(format!("COM init failed: 0x{:08x}", com_result.0));
    }

    if let Err(err) = unsafe { MFStartup(MF_VERSION, 0) } {
        unsafe {
            CoUninitialize();
        }
        return Err(format!("Media Foundation startup failed: 0x{:08x}", err.code().0));
    }
    let mf_started = true;

    let (transform, output_buffer_len, use_dxgi_surface) =
        init_transform(codec_id, width, height, bitrate_kbps, fps).unwrap_or((None, 0, false));
    let encoder_available = transform.is_some();
    Ok(MfInit {
        com_initialized,
        mf_started,
        encoder_available,
        transform,
        output_buffer_len,
        use_dxgi_surface,
    })
}

#[cfg(windows)]
fn init_transform(
    codec_id: CodecId,
    width: i32,
    height: i32,
    bitrate_kbps: u32,
    fps: u32,
) -> Result<(Option<IMFTransform>, u32, bool), String> {
    let output_guid: GUID = match codec_id {
        CodecId::H264 => MFVideoFormat_H264,
        CodecId::H265 => MFVideoFormat_HEVC,
        _ => return Ok((None, 0, false)),
    };

    let output_type = MFT_REGISTER_TYPE_INFO {
        guidMajorType: MFVideoFormat_NV12,
        guidSubtype: output_guid,
    };

    let mut activate: *mut Option<IMFActivate> = std::ptr::null_mut();
    let mut count = 0u32;
    let flags = MFT_ENUM_FLAG_SYNCMFT | MFT_ENUM_FLAG_LOCALMFT;
    unsafe {
        MFTEnumEx(
            MFT_CATEGORY_VIDEO_ENCODER,
            flags,
            None,
            Some(&output_type),
            &mut activate,
            &mut count,
        )
    }
    .map_err(|err| format!("MFEnumEx failed: 0x{:08x}", err.code().0))?;
    if count == 0 || activate.is_null() {
        return Ok((None, 0, false));
    }

    let mut transform: Option<IMFTransform> = None;
    unsafe {
        let slice = std::slice::from_raw_parts_mut(activate, count as usize);
        for item in slice.iter_mut() {
            if let Some(activate_item) = item.take() {
                if transform.is_none() {
                    if let Ok(instance) = activate_item.ActivateObject::<IMFTransform>() {
                        transform = Some(instance);
                    }
                }
                drop(activate_item);
            }
        }
        CoTaskMemFree(Some(activate as *const _));
    }

    let Some(transform) = transform else {
        return Ok((None, 0, false));
    };

    let mut use_dxgi_surface = false;
    let input_type = build_input_type(MFVideoFormat_ARGB32, width, height, fps)?;
    let input_result = unsafe { transform.SetInputType(0, &input_type, 0) };
    if input_result.is_ok() {
        use_dxgi_surface = true;
    } else {
        let nv12_input = build_input_type(MFVideoFormat_NV12, width, height, fps)?;
        unsafe {
            transform
                .SetInputType(0, &nv12_input, 0)
                .map_err(|err| format!("MF SetInputType failed: 0x{:08x}", err.code().0))?;
        }
    }
    let output_type = build_output_type(codec_id, width, height, fps, bitrate_kbps)?;
    unsafe {
        transform
            .SetOutputType(0, &output_type, 0)
            .map_err(|err| format!("MF SetOutputType failed: 0x{:08x}", err.code().0))?;
        let _ = transform.ProcessMessage(MFT_MESSAGE_COMMAND_FLUSH, 0);
        let _ = transform.ProcessMessage(MFT_MESSAGE_COMMAND_DRAIN, 0);
        let _ = transform.ProcessMessage(MFT_MESSAGE_NOTIFY_BEGIN_STREAMING, 0);
        let _ = transform.ProcessMessage(MFT_MESSAGE_NOTIFY_START_OF_STREAM, 0);
    }

    let output_buffer_len = get_output_buffer_len(&transform);
    Ok((Some(transform), output_buffer_len, use_dxgi_surface))
}

#[cfg(windows)]
fn build_input_type(subtype: GUID, width: i32, height: i32, fps: u32) -> Result<IMFMediaType, String> {
    let media_type = unsafe { MFCreateMediaType() }
        .map_err(|err| format!("MFCreateMediaType failed: 0x{:08x}", err.code().0))?;
    unsafe {
        media_type
            .SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video)
            .map_err(|err| format!("MF Set major type failed: 0x{:08x}", err.code().0))?;
        media_type
            .SetGUID(&MF_MT_SUBTYPE, &subtype)
            .map_err(|err| format!("MF Set subtype failed: 0x{:08x}", err.code().0))?;
        set_attribute_size(&media_type, &MF_MT_FRAME_SIZE, width as u32, height as u32)?;
        set_attribute_ratio(&media_type, &MF_MT_FRAME_RATE, fps, 1)?;
        set_attribute_ratio(&media_type, &MF_MT_PIXEL_ASPECT_RATIO, 1, 1)?;
        media_type
            .SetUINT32(&MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive.0 as u32)
            .map_err(|err| format!("MF Set interlace failed: 0x{:08x}", err.code().0))?;
    }
    Ok(media_type)
}

#[cfg(windows)]
fn build_output_type(
    codec_id: CodecId,
    width: i32,
    height: i32,
    fps: u32,
    bitrate_kbps: u32,
) -> Result<IMFMediaType, String> {
    let output_guid = match codec_id {
        CodecId::H264 => MFVideoFormat_H264,
        CodecId::H265 => MFVideoFormat_HEVC,
        _ => return Err("Unsupported output codec".to_string()),
    };
    let media_type = unsafe { MFCreateMediaType() }
        .map_err(|err| format!("MFCreateMediaType failed: 0x{:08x}", err.code().0))?;
    unsafe {
        media_type
            .SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video)
            .map_err(|err| format!("MF Set major type failed: 0x{:08x}", err.code().0))?;
        media_type
            .SetGUID(&MF_MT_SUBTYPE, &output_guid)
            .map_err(|err| format!("MF Set subtype failed: 0x{:08x}", err.code().0))?;
        set_attribute_size(&media_type, &MF_MT_FRAME_SIZE, width as u32, height as u32)?;
        set_attribute_ratio(&media_type, &MF_MT_FRAME_RATE, fps, 1)?;
        set_attribute_ratio(&media_type, &MF_MT_PIXEL_ASPECT_RATIO, 1, 1)?;
        media_type
            .SetUINT32(&MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive.0 as u32)
            .map_err(|err| format!("MF Set interlace failed: 0x{:08x}", err.code().0))?;
        media_type
            .SetUINT32(&MF_MT_AVG_BITRATE, bitrate_kbps.saturating_mul(1000))
            .map_err(|err| format!("MF Set bitrate failed: 0x{:08x}", err.code().0))?;
    }
    Ok(media_type)
}

#[cfg(windows)]
fn get_output_buffer_len(transform: &IMFTransform) -> u32 {
    if let Ok(info) = unsafe { transform.GetOutputStreamInfo(0) } {
        if info.cbSize > 0 {
            return info.cbSize;
        }
    }
    0
}

#[cfg(windows)]
fn drain_output(transform: &IMFTransform, output_buffer_len: u32) -> Result<Vec<u8>, String> {
    let sample = unsafe { MFCreateSample() }
        .map_err(|err| format!("MFCreateSample failed: 0x{:08x}", err.code().0))?;
    let buffer = unsafe { MFCreateMemoryBuffer(output_buffer_len.max(1024)) }
        .map_err(|err| format!("MFCreateMemoryBuffer failed: 0x{:08x}", err.code().0))?;
    unsafe {
        sample
            .AddBuffer(&buffer)
            .map_err(|err| format!("MF AddBuffer failed: 0x{:08x}", err.code().0))?;
    }

    let mut output = MFT_OUTPUT_DATA_BUFFER {
        dwStreamID: 0,
        pSample: ManuallyDrop::new(Some(sample)),
        dwStatus: 0,
        pEvents: ManuallyDrop::new(None),
    };
    let mut status = 0u32;
    let output_result = unsafe { transform.ProcessOutput(0, std::slice::from_mut(&mut output), &mut status) };
    if let Err(err) = output_result {
        if err.code() == MF_E_TRANSFORM_NEED_MORE_INPUT {
            return Ok(Vec::new());
        }
        return Err(format!("MF ProcessOutput failed: 0x{:08x}", err.code().0));
    }

    let sample = unsafe {
        let ptr = &output.pSample as *const ManuallyDrop<Option<windows::Win32::Media::MediaFoundation::IMFSample>>
            as *const Option<windows::Win32::Media::MediaFoundation::IMFSample>;
        (*ptr).clone()
    };
    let Some(sample) = sample else {
        return Ok(Vec::new());
    };
    let buffer = unsafe { sample.GetBufferByIndex(0) }
        .map_err(|err| format!("MF GetBuffer failed: 0x{:08x}", err.code().0))?;
    unsafe {
        let mut data: *mut u8 = std::ptr::null_mut();
        let mut max_len = 0u32;
        let mut current_len = 0u32;
        buffer
            .Lock(&mut data, Some(&mut max_len), Some(&mut current_len))
            .map_err(|err| format!("MF buffer lock failed: 0x{:08x}", err.code().0))?;
        let slice = if data.is_null() || current_len == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(data as *const u8, current_len as usize)
        };
        let payload = slice.to_vec();
        buffer
            .Unlock()
            .map_err(|err| format!("MF buffer unlock failed: 0x{:08x}", err.code().0))?;
        Ok(payload)
    }
}

#[cfg(windows)]
impl MfEncoder {
    fn encode_mf_frame(&mut self) -> Option<(Vec<u8>, u64)> {
        let transform = self.transform.as_ref()?;
        let buffer = if self.use_dxgi_surface {
            match capture::capture_dxgi_surface(self.width, self.height) {
                Ok(frame) => {
                    let buffer = unsafe {
                        MFCreateDXGISurfaceBuffer(
                            &ID3D11Texture2D::IID,
                            &frame.texture,
                            0,
                            false,
                        )
                    };
                    match buffer {
                        Ok(buffer) => buffer,
                        Err(err) => {
                            self.last_error =
                                Some(format!("MFCreateDXGISurfaceBuffer failed: 0x{:08x}", err.code().0));
                            return None;
                        }
                    }
                }
                Err(err) => {
                    self.last_error = Some(err);
                    return None;
                }
            }
        } else {
            let nv12 = capture::capture_nv12(self.width, self.height).ok();
            match create_nv12_sample_with_data(self.width, self.height, nv12.as_deref()) {
                Ok(buffer) => buffer,
                Err(err) => {
                    self.last_error = Some(err);
                    return None;
                }
            }
        };
        let sample = match unsafe { MFCreateSample() } {
            Ok(sample) => sample,
            Err(err) => {
                self.last_error = Some(format!("MFCreateSample failed: 0x{:08x}", err.code().0));
                return None;
            }
        };
        if unsafe { sample.AddBuffer(&buffer) }.is_err() {
            self.last_error = Some("MF AddBuffer failed".to_string());
            return None;
        }
        let timestamp = estimate_timestamp_100ns(self.frame_index, self.fps);
        self.frame_index = self.frame_index.wrapping_add(1);
        let _ = unsafe { sample.SetSampleTime(timestamp as i64) };
        let frame_duration = (10_000_000u64 / self.fps.max(1) as u64) as i64;
        let _ = unsafe { sample.SetSampleDuration(frame_duration) };
        if unsafe { transform.ProcessInput(0, &sample, 0) }.is_err() {
            self.last_error = Some("MF ProcessInput failed".to_string());
            return None;
        }
        match drain_output(transform, self.output_buffer_len) {
            Ok(payload) => {
                if !payload.is_empty() {
                    self.last_error = None;
                }
                Some((payload, timestamp))
            }
            Err(err) => {
                self.last_error = Some(err);
                None
            }
        }
    }

    pub fn take_last_error(&mut self) -> Option<String> {
        self.last_error.take()
    }
}

#[cfg(not(windows))]
impl MfEncoder {
    pub fn take_last_error(&mut self) -> Option<String> {
        None
    }
}

#[cfg(windows)]
fn create_nv12_sample_with_data(
    width: i32,
    height: i32,
    data: Option<&[u8]>,
) -> Result<IMFMediaBuffer, String> {
    let frame_size = (width.max(1) as usize) * (height.max(1) as usize);
    let buffer_len = frame_size + (frame_size / 2);
    let buffer = unsafe { MFCreateMemoryBuffer(buffer_len as u32) }
        .map_err(|err| format!("MFCreateMemoryBuffer failed: 0x{:08x}", err.code().0))?;
    unsafe {
        let mut ptr: *mut u8 = std::ptr::null_mut();
        let mut max_len = 0u32;
        let mut current_len = 0u32;
        buffer
            .Lock(&mut ptr, Some(&mut max_len), Some(&mut current_len))
            .map_err(|err| format!("MF buffer lock failed: 0x{:08x}", err.code().0))?;
        if !ptr.is_null() {
            if let Some(source) = data {
                let copy_len = source.len().min(buffer_len);
                std::ptr::copy_nonoverlapping(source.as_ptr(), ptr, copy_len);
                if copy_len < buffer_len {
                    std::ptr::write_bytes(ptr.add(copy_len), 0u8, buffer_len - copy_len);
                }
            } else {
                std::ptr::write_bytes(ptr, 0u8, buffer_len);
            }
        }
        buffer
            .Unlock()
            .map_err(|err| format!("MF buffer unlock failed: 0x{:08x}", err.code().0))?;
        buffer
            .SetCurrentLength(buffer_len as u32)
            .map_err(|err| format!("MF buffer length failed: 0x{:08x}", err.code().0))?;
    }
    Ok(buffer)
}

#[cfg(windows)]
fn set_attribute_size(
    media_type: &IMFMediaType,
    key: &GUID,
    width: u32,
    height: u32,
) -> Result<(), String> {
    let value = ((width as u64) << 32) | height as u64;
    unsafe {
        media_type
            .SetUINT64(key, value)
            .map_err(|err| format!("MF Set size failed: 0x{:08x}", err.code().0))
    }
}

#[cfg(windows)]
fn set_attribute_ratio(
    media_type: &IMFMediaType,
    key: &GUID,
    numerator: u32,
    denominator: u32,
) -> Result<(), String> {
    let value = ((numerator as u64) << 32) | denominator as u64;
    unsafe {
        media_type
            .SetUINT64(key, value)
            .map_err(|err| format!("MF Set ratio failed: 0x{:08x}", err.code().0))
    }
}

fn estimate_timestamp_100ns(frame_index: u64, fps: u32) -> u64 {
    let fps = fps.max(1) as u64;
    frame_index.saturating_mul(10_000_000 / fps)
}
