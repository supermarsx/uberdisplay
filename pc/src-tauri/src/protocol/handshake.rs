#[derive(Debug, thiserror::Error)]
pub enum HandshakeError {
    #[error("handshake version must be between 0 and 999")]
    VersionOutOfRange,
}

const HANDSHAKE_BASE: &str = "KELOCUBE_MIRR_";
const HANDSHAKE_VERSION_LEN: usize = 3;

pub fn build_host_handshake(version: u16) -> Result<Vec<u8>, HandshakeError> {
    if version > 999 {
        return Err(HandshakeError::VersionOutOfRange);
    }

    let mut buffer = Vec::with_capacity(HANDSHAKE_BASE.len() + HANDSHAKE_VERSION_LEN + 1);
    buffer.extend_from_slice(HANDSHAKE_BASE.as_bytes());
    buffer.extend_from_slice(format!("{:0width$}", version, width = HANDSHAKE_VERSION_LEN).as_bytes());
    buffer.push(0);
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_handshake_with_null_terminator() {
        let bytes = build_host_handshake(4).expect("valid version");
        assert_eq!(bytes, b"KELOCUBE_MIRR_004\0");
    }

    #[test]
    fn rejects_versions_out_of_range() {
        let err = build_host_handshake(1000).expect_err("should fail");
        assert!(matches!(err, HandshakeError::VersionOutOfRange));
    }
}
