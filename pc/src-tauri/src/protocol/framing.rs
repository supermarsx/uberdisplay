pub const MAX_STREAM_CHUNK_LEN: usize = u16::MAX as usize;

pub fn encode_stream_packet(data_type: u8, payload: &[u8]) -> Vec<u8> {
    let packet_len = 1 + payload.len();
    let mut buffer = Vec::with_capacity(4 + packet_len);
    buffer.extend_from_slice(&(packet_len as u32).to_le_bytes());
    buffer.push(data_type);
    buffer.extend_from_slice(payload);
    buffer
}

pub fn write_stream_chunks(stream_id: u8, packet: &[u8], out: &mut Vec<u8>) {
    let mut offset = 0;
    while offset < packet.len() {
        let remaining = packet.len() - offset;
        let chunk_len = remaining.min(MAX_STREAM_CHUNK_LEN);
        out.push(stream_id);
        out.extend_from_slice(&(chunk_len as u16).to_le_bytes());
        out.extend_from_slice(&packet[offset..offset + chunk_len]);
        offset += chunk_len;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_stream_packet_with_length_prefix() {
        let packet = encode_stream_packet(3, &[1, 2, 3]);
        assert_eq!(packet[0..4], 4u32.to_le_bytes());
        assert_eq!(packet[4], 3);
        assert_eq!(&packet[5..], &[1, 2, 3]);
    }

    #[test]
    fn splits_packets_into_multiple_chunks() {
        let packet = vec![0u8; MAX_STREAM_CHUNK_LEN + 10];
        let mut out = Vec::new();
        write_stream_chunks(2, &packet, &mut out);

        let first_len = u16::from_le_bytes([out[1], out[2]]) as usize;
        assert_eq!(out[0], 2);
        assert_eq!(first_len, MAX_STREAM_CHUNK_LEN);
        let second_offset = 1 + 2 + first_len;
        assert_eq!(out[second_offset], 2);
        let second_len = u16::from_le_bytes([out[second_offset + 1], out[second_offset + 2]]) as usize;
        assert_eq!(second_len, 10);
    }
}
