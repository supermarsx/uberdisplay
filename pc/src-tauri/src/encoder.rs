#![allow(dead_code)]

use crate::codec::CodecId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub fn detect_backends() -> Vec<EncoderBackend> {
    #[cfg(windows)]
    {
        // TODO: replace stubs with real detection (DXGI/NVENC/AMF/QSV/MediaFoundation).
        vec![
            EncoderBackend::Nvenc,
            EncoderBackend::Amf,
            EncoderBackend::Qsv,
            EncoderBackend::MediaFoundation,
            EncoderBackend::Software,
        ]
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
