use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use std::thread;
use std::time::{Duration, Instant};

use crate::codec::CodecId;
use crate::host_transport;
use crate::mf_encoder::MfEncoder;
use crate::protocol::packets::{build_frame_packet, FramePacket};

static STREAM_RUNNING: OnceLock<AtomicBool> = OnceLock::new();

fn running_flag() -> &'static AtomicBool {
    STREAM_RUNNING.get_or_init(|| AtomicBool::new(false))
}

pub fn start_streaming(
    codec_id: CodecId,
    encoder_id: i32,
    bitrate_kbps: u32,
    fps: u32,
    keyframe_interval: u32,
) -> Result<(), String> {
    if running_flag().swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    let encoder = MfEncoder::new(codec_id, 0, 0, bitrate_kbps, fps, keyframe_interval)?;

    thread::spawn(move || {
        let mut awaiting_ack = false;
        let mut last_send = Instant::now();
        let max_wait_ms = (1000 / fps.max(1)).saturating_mul(2).max(8) as u64;
        while running_flag().load(Ordering::SeqCst) {
            if awaiting_ack {
                let mut ack_received = false;
                if let Some(done) = host_transport::take_last_frame_done() {
                    if done == encoder_id {
                        ack_received = true;
                    }
                }
                if ack_received || last_send.elapsed().as_millis() as u64 >= max_wait_ms {
                    awaiting_ack = false;
                } else {
                    thread::sleep(Duration::from_millis(4));
                    continue;
                }
            }

            let payload = encoder.encode_dummy_frame();
            let packet = build_frame_packet(FramePacket {
                frame_meta: 0,
                h264_bytes: &payload,
            });
            let _ = host_transport::send_framed_packet(&packet);
            awaiting_ack = true;
            last_send = Instant::now();
            let frame_delay = (1000 / fps.max(1)).max(4);
            thread::sleep(Duration::from_millis(frame_delay as u64));
        }
    });

    Ok(())
}

pub fn stop_streaming() {
    running_flag().store(false, Ordering::SeqCst);
}
