#![allow(dead_code)]

use crate::codec::CodecId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum EncoderBackend {
    Nvenc,
    Amf,
    Qsv,
    MediaFoundation,
    Software,
}

#[derive(Debug, Clone)]
pub struct EncoderCapability {
    pub codec: CodecId,
    pub backends: Vec<EncoderBackend>,
}

#[cfg(windows)]
fn detect_gpu_vendors() -> Vec<GpuVendor> {
    use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory1, IDXGIFactory1};

    let mut vendors = Vec::new();
    let factory: IDXGIFactory1 = match unsafe { CreateDXGIFactory1() } {
        Ok(f) => f,
        Err(_) => return vendors,
    };
    let mut index = 0u32;
    loop {
        let adapter = match unsafe { factory.EnumAdapters1(index) } {
            Ok(a) => a,
            Err(_) => break,
        };
        index += 1;
        let desc = match unsafe { adapter.GetDesc1() } {
            Ok(d) => d,
            Err(_) => continue,
        };
        let vendor_id = desc.VendorId;
        let vendor = match vendor_id {
            0x10DE => GpuVendor::Nvidia,
            0x1002 => GpuVendor::Amd,
            0x8086 => GpuVendor::Intel,
            _ => continue,
        };
        if !vendors.contains(&vendor) {
            vendors.push(vendor);
        }
    }
    vendors
}

#[cfg(windows)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
}

#[cfg(windows)]
fn detect_mf_encoder_available() -> bool {
    use windows::Win32::Media::MediaFoundation::{
        MFTEnumEx, MFT_CATEGORY_VIDEO_ENCODER, MFT_ENUM_FLAG_LOCALMFT,
        MFT_ENUM_FLAG_SYNCMFT, MFT_REGISTER_TYPE_INFO, MFVideoFormat_H264,
        MFMediaType_Video,
    };
    use windows::Win32::System::Com::CoTaskMemFree;

    let output_type = MFT_REGISTER_TYPE_INFO {
        guidMajorType: MFMediaType_Video,
        guidSubtype: MFVideoFormat_H264,
    };
    let flags = MFT_ENUM_FLAG_SYNCMFT | MFT_ENUM_FLAG_LOCALMFT;
    let mut activate = std::ptr::null_mut();
    let mut count = 0u32;
    let result = unsafe {
        MFTEnumEx(
            MFT_CATEGORY_VIDEO_ENCODER,
            flags,
            None,
            Some(&output_type),
            &mut activate,
            &mut count,
        )
    };
    if !activate.is_null() {
        unsafe { CoTaskMemFree(Some(activate as *const _)) };
    }
    result.is_ok() && count > 0
}

pub fn detect_backends() -> Vec<EncoderBackend> {
    #[cfg(windows)]
    {
        let mut backends = Vec::new();
        let vendors = detect_gpu_vendors();
        if vendors.contains(&GpuVendor::Nvidia) {
            backends.push(EncoderBackend::Nvenc);
        }
        if vendors.contains(&GpuVendor::Amd) {
            backends.push(EncoderBackend::Amf);
        }
        if vendors.contains(&GpuVendor::Intel) {
            backends.push(EncoderBackend::Qsv);
        }
        if detect_mf_encoder_available() {
            backends.push(EncoderBackend::MediaFoundation);
        }
        backends.push(EncoderBackend::Software);
        backends
    }
    #[cfg(not(windows))]
    {
        vec![EncoderBackend::Software]
    }
}

pub fn backend_priority() -> Vec<EncoderBackend> {
    vec![
        EncoderBackend::Nvenc,
        EncoderBackend::Amf,
        EncoderBackend::Qsv,
        EncoderBackend::MediaFoundation,
        EncoderBackend::Software,
    ]
}

pub fn select_backend(preferred: Option<EncoderBackend>) -> EncoderBackend {
    let available = detect_backends();
    if let Some(choice) = preferred {
        if available.contains(&choice) {
            return choice;
        }
    }
    for backend in backend_priority() {
        if available.contains(&backend) {
            return backend;
        }
    }
    EncoderBackend::Software
}

pub fn codec_priority() -> Vec<CodecId> {
    vec![CodecId::H265, CodecId::Av1, CodecId::H264, CodecId::Vp9]
}

pub fn discover_capabilities() -> Vec<EncoderCapability> {
    let backends = detect_backends();
    let mut capabilities = Vec::new();
    let mf_codecs = [CodecId::H264, CodecId::H265];
    let sw_codecs = [CodecId::H264];

    for codec in codec_priority() {
        let mut codec_backends = Vec::new();
        for backend in &backends {
            let supported = match backend {
                EncoderBackend::Nvenc
                | EncoderBackend::Amf
                | EncoderBackend::Qsv => {
                    matches!(codec, CodecId::H264 | CodecId::H265)
                }
                EncoderBackend::MediaFoundation => mf_codecs.contains(&codec),
                EncoderBackend::Software => sw_codecs.contains(&codec),
            };
            if supported {
                codec_backends.push(*backend);
            }
        }
        if !codec_backends.is_empty() {
            capabilities.push(EncoderCapability {
                codec,
                backends: codec_backends,
            });
        }
    }
    capabilities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backend_priority_starts_with_nvenc() {
        let priority = backend_priority();
        assert_eq!(priority[0], EncoderBackend::Nvenc);
    }

    #[test]
    fn backend_priority_ends_with_software() {
        let priority = backend_priority();
        assert_eq!(*priority.last().unwrap(), EncoderBackend::Software);
    }

    #[test]
    fn detect_backends_always_includes_software() {
        let backends = detect_backends();
        assert!(backends.contains(&EncoderBackend::Software));
    }

    #[test]
    fn select_backend_returns_preferred_if_available() {
        let result = select_backend(Some(EncoderBackend::Software));
        assert_eq!(result, EncoderBackend::Software);
    }

    #[test]
    fn select_backend_falls_through_when_preferred_unavailable() {
        let result = select_backend(None);
        let backends = detect_backends();
        assert!(backends.contains(&result), "selected backend must be in detected list");
    }

    #[test]
    fn codec_priority_follows_spec_order() {
        let priority = codec_priority();
        assert_eq!(priority[0], CodecId::H265);
        assert_eq!(priority[1], CodecId::Av1);
        assert_eq!(priority[2], CodecId::H264);
        assert_eq!(priority[3], CodecId::Vp9);
    }

    #[test]
    fn discover_capabilities_returns_at_least_h264() {
        let caps = discover_capabilities();
        let h264 = caps.iter().find(|c| c.codec == CodecId::H264);
        assert!(h264.is_some(), "H.264 should always be available via Software");
        assert!(!h264.unwrap().backends.is_empty());
    }
}
