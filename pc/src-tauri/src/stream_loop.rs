use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use std::thread;
use std::time::{Duration, Instant};

use crate::codec::CodecId;
use crate::app_state::SessionStats;
use crate::host_transport;
use crate::mf_encoder::MfEncoder;
use crate::protocol::packets::{build_frame_packet, FramePacket};
use crate::session_state;

static STREAM_RUNNING: OnceLock<AtomicBool> = OnceLock::new();

fn running_flag() -> &'static AtomicBool {
    STREAM_RUNNING.get_or_init(|| AtomicBool::new(false))
}

pub fn start_streaming(
    codec_id: CodecId,
    encoder_id: i32,
    width: i32,
    height: i32,
    bitrate_kbps: u32,
    fps: u32,
    keyframe_interval: u32,
) -> Result<(), String> {
    if running_flag().swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    thread::spawn(move || {
        let mut encoder =
            match MfEncoder::new(codec_id, width, height, bitrate_kbps, fps, keyframe_interval) {
                Ok(encoder) => encoder,
                Err(_) => return,
            };
        let mut awaiting_ack = false;
        let mut last_send = Instant::now();
        let mut last_timestamp: Option<u64> = None;
        let mut last_stats_at = Instant::now();
        let mut window_bytes = 0u64;
        let mut window_frames = 0u32;
        let mut frames_sent = 0u64;
        let mut frames_acked = 0u64;
        let mut mf_failures = 0u32;
        let max_wait_ms = (1000 / fps.max(1)).saturating_mul(2).max(8) as u64;
        while running_flag().load(Ordering::SeqCst) {
            if awaiting_ack {
                let mut ack_received = false;
                if let Some(done) = host_transport::take_last_frame_done() {
                    if done == encoder_id {
                        ack_received = true;
                    }
                }
                if ack_received {
                    frames_acked = frames_acked.saturating_add(1);
                }
                if !(ack_received || last_send.elapsed().as_millis() as u64 >= max_wait_ms) {
                    thread::sleep(Duration::from_millis(4));
                    continue;
                }
            }

            let (payload, timestamp_100ns) = encoder.encode_frame();
            if let Some(err) = encoder.take_last_error() {
                mf_failures = mf_failures.saturating_add(1);
                if mf_failures >= 3 {
                    session_state::update_lifecycle(crate::app_state::SessionLifecycle::Error);
                }
                let _ = err;
            } else if mf_failures > 0 {
                mf_failures = 0;
                session_state::update_lifecycle(crate::app_state::SessionLifecycle::Streaming);
            }
            let payload_len = payload.len() as u32;
            let packet = build_frame_packet(FramePacket {
                frame_meta: 0,
                timestamp_100ns,
                h264_bytes: &payload,
            });
            let _ = host_transport::send_framed_packet(&packet);
            frames_sent = frames_sent.saturating_add(1);
            window_frames = window_frames.saturating_add(1);
            window_bytes = window_bytes.saturating_add(payload.len() as u64);
            awaiting_ack = true;
            last_send = Instant::now();

            if last_stats_at.elapsed() >= Duration::from_millis(1000) {
                let elapsed = last_stats_at.elapsed().as_secs_f32().max(0.001);
                let fps_estimate = window_frames as f32 / elapsed;
                let bitrate_kbps =
                    ((window_bytes as f32 * 8.0) / 1000.0 / elapsed).round() as u32;
                let (dxgi_timeouts, dxgi_access_lost, dxgi_failures, dxgi_last_bytes) =
                    crate::capture::dxgi_stats_snapshot().unwrap_or((0, 0, 0, 0));
                let (capture_path, capture_scale) =
                    crate::capture::capture_info_snapshot()
                        .unwrap_or(("Unknown".to_string(), "Unknown".to_string()));
                session_state::update_stats(SessionStats {
                    fps: (fps_estimate * 10.0).round() / 10.0,
                    bitrate_kbps,
                    frames_sent,
                    frames_acked,
                    last_frame_bytes: payload_len,
                    queue_depth: if awaiting_ack { 1 } else { 0 },
                    dxgi_timeouts,
                    dxgi_access_lost,
                    dxgi_failures,
                    dxgi_last_bytes,
                    capture_path,
                    capture_scale,
                });
                window_bytes = 0;
                window_frames = 0;
                last_stats_at = Instant::now();
            }
            let frame_delay = if let Some(ts) = timestamp_100ns {
                let delay = if let Some(prev) = last_timestamp {
                    ts.saturating_sub(prev)
                } else {
                    0
                };
                last_timestamp = Some(ts);
                if delay > 0 {
                    (delay / 10_000).max(4)
                } else {
                    (1000 / fps.max(1)).max(4) as u64
                }
            } else {
                (1000 / fps.max(1)).max(4) as u64
            };
            thread::sleep(Duration::from_millis(frame_delay));
        }

        session_state::reset_stats();
    });

    Ok(())
}

pub fn stop_streaming() {
    running_flag().store(false, Ordering::SeqCst);
}
