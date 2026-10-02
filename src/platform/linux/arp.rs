use std::fs;

pub fn read_arp_table() -> std::io::Result<String>{
    fs::read_to_string("/proc/net/arp")
}
