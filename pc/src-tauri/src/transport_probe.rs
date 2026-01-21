use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

use crate::app_state::TransportStatus;

const TCP_PORT: u16 = 1445;

pub fn probe_transport_status() -> TransportStatus {
    TransportStatus {
        tcp_listening: probe_tcp_listener(),
        tcp_connections: 0,
        aoap_attached: false,
    }
}

fn probe_tcp_listener() -> bool {
    let addr = SocketAddr::from(([127, 0, 0, 1], TCP_PORT));
    TcpStream::connect_timeout(&addr, Duration::from_millis(200)).is_ok()
}
