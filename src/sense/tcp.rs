use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub fn is_tcp_open(address: SocketAddr, timeout: Duration) -> bool {
    TcpStream::connect_timeout(&address, timeout).is_ok()
}
