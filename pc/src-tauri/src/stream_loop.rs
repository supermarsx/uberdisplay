use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

use crate::codec::CodecId;
use crate::host_transport;
use crate::mf_encoder::MfEncoder;
use crate::protocol::packets::{build_frame_packet, FramePacket};

static STREAM_RUNNING: OnceLock<AtomicBool> = OnceLock::new();

fn running_flag() -> &'static AtomicBool {
    STREAM_RUNNING.get_or_init(|| AtomicBool::new(false))
}

pub fn start_streaming(codec_id: CodecId, encoder_id: i32) -> Result<(), String> {
    if running_flag().swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    let encoder = MfEncoder::new(codec_id, 0, 0)?;

    thread::spawn(move || {
        let mut awaiting_ack = false;
        while running_flag().load(Ordering::SeqCst) {
            if awaiting_ack {
                if let Some(done) = host_transport::take_last_frame_done() {
                    if done == encoder_id {
                        awaiting_ack = false;
                    }
                }
                thread::sleep(Duration::from_millis(4));
                continue;
            }

            let payload = encoder.encode_dummy_frame();
            let packet = build_frame_packet(FramePacket {
                frame_meta: 0,
                h264_bytes: &payload,
            });
            let _ = host_transport::send_framed_packet(&packet);
            awaiting_ack = true;
            thread::sleep(Duration::from_millis(16));
        }
    });

    Ok(())
}

pub fn stop_streaming() {
    running_flag().store(false, Ordering::SeqCst);
}
