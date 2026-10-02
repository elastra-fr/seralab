mod sense;

use std::net::SocketAddr;
use std::time::Duration;

use sense::tcp::is_tcp_open;

fn main() {
let address: SocketAddr = "127.0.0.1:80".parse().expect("Invalid address");
let is_open = is_tcp_open(address, Duration::from_secs(5));

println!("Is TCP open: {}", is_open);
}

