use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread;

use crate::protocol::framing::write_stream_chunks;
use crate::protocol::handshake::build_host_handshake;
use crate::protocol::packets::{parse_client_packet, ClientPacket};
use crate::session_state;
use crate::app_state::SessionLifecycle;

static TCP_STREAM: OnceLock<Mutex<Option<TcpStream>>> = OnceLock::new();
static LAST_CLIENT_CODEC_MASK: OnceLock<Mutex<Option<u32>>> = OnceLock::new();
static LAST_FRAME_DONE: OnceLock<Mutex<Option<i32>>> = OnceLock::new();
static CONNECTED: OnceLock<AtomicBool> = OnceLock::new();
static RECONNECT_ENABLED: OnceLock<AtomicBool> = OnceLock::new();
static RECONNECTING: OnceLock<AtomicBool> = OnceLock::new();
static LAST_CONNECT: OnceLock<Mutex<Option<ConnectInfo>>> = OnceLock::new();
static LAST_CAPS: OnceLock<Mutex<Option<Vec<u8>>>> = OnceLock::new();
static LAST_CONFIGURE: OnceLock<Mutex<Option<Vec<u8>>>> = OnceLock::new();

fn stream_store() -> &'static Mutex<Option<TcpStream>> {
    TCP_STREAM.get_or_init(|| Mutex::new(None))
}

fn codec_mask_store() -> &'static Mutex<Option<u32>> {
    LAST_CLIENT_CODEC_MASK.get_or_init(|| Mutex::new(None))
}

fn frame_done_store() -> &'static Mutex<Option<i32>> {
    LAST_FRAME_DONE.get_or_init(|| Mutex::new(None))
}

fn connected_flag() -> &'static AtomicBool {
    CONNECTED.get_or_init(|| AtomicBool::new(false))
}

fn reconnect_enabled_flag() -> &'static AtomicBool {
    RECONNECT_ENABLED.get_or_init(|| AtomicBool::new(false))
}

fn reconnecting_flag() -> &'static AtomicBool {
    RECONNECTING.get_or_init(|| AtomicBool::new(false))
}

fn last_connect_store() -> &'static Mutex<Option<ConnectInfo>> {
    LAST_CONNECT.get_or_init(|| Mutex::new(None))
}

fn last_caps_store() -> &'static Mutex<Option<Vec<u8>>> {
    LAST_CAPS.get_or_init(|| Mutex::new(None))
}

fn last_configure_store() -> &'static Mutex<Option<Vec<u8>>> {
    LAST_CONFIGURE.get_or_init(|| Mutex::new(None))
}

#[derive(Debug, Clone)]
struct ConnectInfo {
    host: String,
    port: u16,
}

pub fn set_last_session(
    host: String,
    port: u16,
    caps_packet: Vec<u8>,
    configure_packet: Vec<u8>,
) {
    if let Ok(mut guard) = last_connect_store().lock() {
        *guard = Some(ConnectInfo { host, port });
    }
    if let Ok(mut guard) = last_caps_store().lock() {
        *guard = Some(caps_packet);
    }
    if let Ok(mut guard) = last_configure_store().lock() {
        *guard = Some(configure_packet);
    }
}

pub fn connect(addr: &str, port: u16) -> Result<(), String> {
    let target = format!("{addr}:{port}");
    let mut addrs = target
        .to_socket_addrs()
        .map_err(|err| err.to_string())?;
    let socket_addr = addrs.next().ok_or_else(|| "No address resolved".to_string())?;
    let mut stream = TcpStream::connect(socket_addr).map_err(|err| err.to_string())?;
    stream
        .set_nodelay(true)
        .map_err(|err| err.to_string())?;

    let handshake = build_host_handshake(4).map_err(|err| err.to_string())?;
    stream.write_all(&handshake).map_err(|err| err.to_string())?;

    let reader_stream = stream.try_clone().map_err(|err| err.to_string())?;
    start_reader(reader_stream);

    let mut lock = stream_store().lock().map_err(|_| "Lock poisoned".to_string())?;
    *lock = Some(stream);
    connected_flag().store(true, Ordering::SeqCst);
    reconnect_enabled_flag().store(true, Ordering::SeqCst);
    Ok(())
}

pub fn disconnect() -> Result<(), String> {
    let mut lock = stream_store().lock().map_err(|_| "Lock poisoned".to_string())?;
    *lock = None;
    connected_flag().store(false, Ordering::SeqCst);
    reconnect_enabled_flag().store(false, Ordering::SeqCst);
    crate::stream_loop::stop_streaming();
    crate::session_state::reset_stats();
    Ok(())
}

pub fn is_connected() -> bool {
    connected_flag().load(Ordering::SeqCst)
}

pub fn send_framed_packet(packet: &[u8]) -> Result<(), String> {
    let mut framed = Vec::with_capacity(4 + packet.len());
    framed.extend_from_slice(&(packet.len() as u32).to_le_bytes());
    framed.extend_from_slice(packet);

    let mut chunked = Vec::with_capacity(framed.len() + 3);
    write_stream_chunks(0, &framed, &mut chunked);

    let mut lock = stream_store().lock().map_err(|_| "Lock poisoned".to_string())?;
    let stream = lock.as_mut().ok_or_else(|| "TCP stream not connected".to_string())?;
    stream.write_all(&chunked).map_err(|err| err.to_string())
}

pub fn take_last_client_codec_mask() -> Option<u32> {
    codec_mask_store().lock().ok().and_then(|mut guard| guard.take())
}

pub fn take_last_frame_done() -> Option<i32> {
    frame_done_store().lock().ok().and_then(|mut guard| guard.take())
}

fn start_reader(mut stream: TcpStream) {
    thread::spawn(move || {
        let mut pending = VecDeque::new();
        let mut stream_buffers: [VecDeque<u8>; 2] = [VecDeque::new(), VecDeque::new()];
        let mut buffer = [0u8; 4096];
        loop {
            let read = match stream.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };
            for byte in &buffer[..read] {
                pending.push_back(*byte);
            }

            while pending.len() >= 3 {
                let stream_id = *pending.get(0).unwrap();
                let len_lo = *pending.get(1).unwrap();
                let len_hi = *pending.get(2).unwrap();
                let chunk_len = u16::from_le_bytes([len_lo, len_hi]) as usize;
                if pending.len() < 3 + chunk_len {
                    break;
                }
                pending.pop_front();
                pending.pop_front();
                pending.pop_front();

                let stream_index = (stream_id as usize).min(stream_buffers.len() - 1);
                for _ in 0..chunk_len {
                    if let Some(byte) = pending.pop_front() {
                        stream_buffers[stream_index].push_back(byte);
                    }
                }

                parse_stream_buffer(&mut stream_buffers[stream_index]);
            }
        }

        if let Ok(mut lock) = stream_store().lock() {
            *lock = None;
        }
        connected_flag().store(false, Ordering::SeqCst);
        crate::stream_loop::stop_streaming();
        crate::session_state::reset_stats();
        if reconnect_enabled_flag().load(Ordering::SeqCst) {
            session_state::update_lifecycle(SessionLifecycle::Error);
            attempt_reconnect();
        } else {
            session_state::update_lifecycle(SessionLifecycle::Idle);
        }
    });
}

fn attempt_reconnect() {
    if reconnecting_flag()
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return;
    }

    thread::spawn(move || {
        let mut attempt = 0;
        let backoff_ms = [1500, 3000, 6000];
        session_state::update_lifecycle(SessionLifecycle::Connecting);
        loop {
            if connected_flag().load(Ordering::SeqCst) {
                break;
            }
            let info = last_connect_store()
                .lock()
                .ok()
                .and_then(|guard| guard.clone());
            let (host, port) = match info {
                Some(info) => (info.host, info.port),
                None => break,
            };

            let connect_result = connect(&host, port);
            if connect_result.is_ok() {
                if let (Some(caps), Some(configure)) = (
                    last_caps_store().lock().ok().and_then(|guard| guard.clone()),
                    last_configure_store()
                        .lock()
                        .ok()
                        .and_then(|guard| guard.clone()),
                ) {
                    let _ = send_framed_packet(&caps);
                    let _ = send_framed_packet(&configure);
                }
                session_state::update_lifecycle(SessionLifecycle::Configured);
                break;
            }

            if attempt >= backoff_ms.len() {
                break;
            }
            let delay = backoff_ms[attempt];
            attempt += 1;
            thread::sleep(std::time::Duration::from_millis(delay));
        }
        if !connected_flag().load(Ordering::SeqCst) {
            session_state::update_lifecycle(SessionLifecycle::Error);
        }
        reconnecting_flag().store(false, Ordering::SeqCst);
    });
}

fn parse_stream_buffer(buffer: &mut VecDeque<u8>) {
    loop {
        if buffer.len() < 4 {
            return;
        }
        let len_bytes = [
            *buffer.get(0).unwrap(),
            *buffer.get(1).unwrap(),
            *buffer.get(2).unwrap(),
            *buffer.get(3).unwrap(),
        ];
        let packet_len = u32::from_le_bytes(len_bytes) as usize;
        if buffer.len() < 4 + packet_len {
            return;
        }
        for _ in 0..4 {
            buffer.pop_front();
        }
        let mut payload = Vec::with_capacity(packet_len);
        for _ in 0..packet_len {
            if let Some(byte) = buffer.pop_front() {
                payload.push(byte);
            }
        }
        if let Ok(packet) = parse_client_packet(&payload) {
            match packet {
                ClientPacket::Capabilities(caps) => {
                    if let Ok(mut guard) = codec_mask_store().lock() {
                        *guard = Some(caps.codec_mask);
                    }
                }
                ClientPacket::FrameDone(frame) => {
                    if let Ok(mut guard) = frame_done_store().lock() {
                        *guard = Some(frame.encoder_id);
                    }
                }
                ClientPacket::Touch(_)
                | ClientPacket::Pen(_)
                | ClientPacket::Keyboard(_) => {
                    let state = crate::session_state::snapshot();
                    if !state.input_permissions.enable_input {
                        continue;
                    }
                    let allowed = match packet {
                        ClientPacket::Touch(_) => state.input_permissions.touch,
                        ClientPacket::Pen(_) => state.input_permissions.pen,
                        ClientPacket::Keyboard(_) => state.input_permissions.keyboard,
                        _ => true,
                    };
                    if !allowed {
                        continue;
                    }
                }
                _ => {}
            }
        }
    }
}
