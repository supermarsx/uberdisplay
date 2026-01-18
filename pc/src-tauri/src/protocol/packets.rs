use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("packet payload too short")]
    PayloadTooShort,
    #[error("touch packet length mismatch")]
    TouchLengthMismatch,
    #[error("unsupported data type {0}")]
    UnsupportedDataType(u8),
}

#[derive(Debug, PartialEq)]
pub struct ConfigurePacket {
    pub width: i32,
    pub height: i32,
    pub host_width: i32,
    pub host_height: i32,
    pub encoder_id: i32,
}

#[derive(Debug, PartialEq)]
pub struct FramePacket<'a> {
    pub frame_meta: u8,
    pub h264_bytes: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct TouchPoint {
    pub pointer_id: u8,
    pub down: bool,
    pub x: i16,
    pub y: i16,
    pub size: i16,
}

#[derive(Debug, PartialEq)]
pub struct TouchPacket {
    pub points: Vec<TouchPoint>,
}

#[derive(Debug, PartialEq)]
pub struct PenPacket {
    pub flags: u8,
    pub x: i16,
    pub y: i16,
    pub pressure: i16,
    pub rotation: i16,
    pub tilt: i16,
}

#[derive(Debug, PartialEq)]
pub struct KeyboardPacket {
    pub down: bool,
    pub key_index: i32,
}

#[derive(Debug, PartialEq)]
pub struct InputKeyPacket {
    pub down: bool,
    pub button_index: u8,
    pub action: i32,
}

#[derive(Debug, PartialEq)]
pub enum ClientPacket {
    Touch(TouchPacket),
    Pen(PenPacket),
    Keyboard(KeyboardPacket),
    InputKey(InputKeyPacket),
}

pub fn build_state_packet(payload: &[u8]) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(1 + payload.len());
    buffer.push(0);
    buffer.extend_from_slice(payload);
    buffer
}

pub fn build_configure_packet(packet: ConfigurePacket) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(1 + 5 * 4);
    buffer.push(1);
    buffer.extend_from_slice(&packet.width.to_le_bytes());
    buffer.extend_from_slice(&packet.height.to_le_bytes());
    buffer.extend_from_slice(&packet.host_width.to_le_bytes());
    buffer.extend_from_slice(&packet.host_height.to_le_bytes());
    buffer.extend_from_slice(&packet.encoder_id.to_le_bytes());
    buffer
}

pub fn build_frame_packet(packet: FramePacket<'_>) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(2 + packet.h264_bytes.len());
    buffer.push(3);
    buffer.push(packet.frame_meta);
    buffer.extend_from_slice(packet.h264_bytes);
    buffer
}

pub fn parse_client_packet(bytes: &[u8]) -> Result<ClientPacket, PacketError> {
    let (data_type, payload) = bytes.split_first().ok_or(PacketError::PayloadTooShort)?;

    match data_type {
        8 => parse_touch_packet(payload).map(ClientPacket::Touch),
        9 => parse_pen_packet(payload).map(ClientPacket::Pen),
        13 => parse_input_key_packet(payload).map(ClientPacket::InputKey),
        15 => parse_keyboard_packet(payload).map(ClientPacket::Keyboard),
        other => Err(PacketError::UnsupportedDataType(*other)),
    }
}

fn parse_touch_packet(payload: &[u8]) -> Result<TouchPacket, PacketError> {
    let (count, rest) = payload.split_first().ok_or(PacketError::PayloadTooShort)?;
    let expected_len = 1 + (*count as usize * 8);
    if payload.len() != expected_len {
        return Err(PacketError::TouchLengthMismatch);
    }

    let mut points = Vec::with_capacity(*count as usize);
    for chunk in rest.chunks_exact(8) {
        points.push(TouchPoint {
            pointer_id: chunk[0],
            down: chunk[1] != 0,
            x: i16::from_le_bytes([chunk[2], chunk[3]]),
            y: i16::from_le_bytes([chunk[4], chunk[5]]),
            size: i16::from_le_bytes([chunk[6], chunk[7]]),
        });
    }

    Ok(TouchPacket { points })
}

fn parse_pen_packet(payload: &[u8]) -> Result<PenPacket, PacketError> {
    if payload.len() != 11 {
        return Err(PacketError::PayloadTooShort);
    }

    Ok(PenPacket {
        flags: payload[0],
        x: i16::from_le_bytes([payload[1], payload[2]]),
        y: i16::from_le_bytes([payload[3], payload[4]]),
        pressure: i16::from_le_bytes([payload[5], payload[6]]),
        rotation: i16::from_le_bytes([payload[7], payload[8]]),
        tilt: i16::from_le_bytes([payload[9], payload[10]]),
    })
}

fn parse_keyboard_packet(payload: &[u8]) -> Result<KeyboardPacket, PacketError> {
    if payload.len() != 5 {
        return Err(PacketError::PayloadTooShort);
    }

    Ok(KeyboardPacket {
        down: payload[0] != 0,
        key_index: i32::from_le_bytes([payload[1], payload[2], payload[3], payload[4]]),
    })
}

fn parse_input_key_packet(payload: &[u8]) -> Result<InputKeyPacket, PacketError> {
    if payload.len() != 6 {
        return Err(PacketError::PayloadTooShort);
    }

    Ok(InputKeyPacket {
        down: payload[0] != 0,
        button_index: payload[1],
        action: i32::from_le_bytes([payload[2], payload[3], payload[4], payload[5]]),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_configure_packet() {
        let packet = build_configure_packet(ConfigurePacket {
            width: 1920,
            height: 1080,
            host_width: 2560,
            host_height: 1440,
            encoder_id: 7,
        });

        assert_eq!(packet[0], 1);
        assert_eq!(i32::from_le_bytes(packet[1..5].try_into().unwrap()), 1920);
        assert_eq!(i32::from_le_bytes(packet[5..9].try_into().unwrap()), 1080);
        assert_eq!(i32::from_le_bytes(packet[9..13].try_into().unwrap()), 2560);
        assert_eq!(i32::from_le_bytes(packet[13..17].try_into().unwrap()), 1440);
        assert_eq!(i32::from_le_bytes(packet[17..21].try_into().unwrap()), 7);
    }

    #[test]
    fn builds_frame_packet_with_meta() {
        let packet = build_frame_packet(FramePacket {
            frame_meta: 2,
            h264_bytes: &[0x01, 0x02],
        });

        assert_eq!(packet, vec![3, 2, 0x01, 0x02]);
    }

    #[test]
    fn parses_touch_packet() {
        let payload = [
            2u8,
            1, 1, 10, 0, 20, 0, 30, 0,
            2, 0, 40, 0, 50, 0, 60, 0,
        ];
        let packet = parse_client_packet(&[8u8].iter().chain(payload.iter()).copied().collect::<Vec<_>>()).unwrap();

        match packet {
            ClientPacket::Touch(touch) => {
                assert_eq!(touch.points.len(), 2);
                assert_eq!(touch.points[0].pointer_id, 1);
                assert_eq!(touch.points[1].down, false);
            }
            _ => panic!("unexpected packet"),
        }
    }

    #[test]
    fn parses_pen_packet() {
        let payload = [1u8, 10, 0, 11, 0, 12, 0, 13, 0, 14, 0];
        let packet = parse_client_packet(&[9u8].iter().chain(payload.iter()).copied().collect::<Vec<_>>()).unwrap();

        match packet {
            ClientPacket::Pen(pen) => {
                assert_eq!(pen.flags, 1);
                assert_eq!(pen.pressure, 12);
            }
            _ => panic!("unexpected packet"),
        }
    }

    #[test]
    fn parses_keyboard_packet() {
        let payload = [1u8, 5, 0, 0, 0];
        let packet = parse_client_packet(&[15u8].iter().chain(payload.iter()).copied().collect::<Vec<_>>()).unwrap();

        assert_eq!(packet, ClientPacket::Keyboard(KeyboardPacket { down: true, key_index: 5 }));
    }

    #[test]
    fn parses_input_key_packet() {
        let payload = [1u8, 2, 9, 0, 0, 0];
        let packet = parse_client_packet(&[13u8].iter().chain(payload.iter()).copied().collect::<Vec<_>>()).unwrap();

        assert_eq!(
            packet,
            ClientPacket::InputKey(InputKeyPacket {
                down: true,
                button_index: 2,
                action: 9,
            })
        );
    }
}
