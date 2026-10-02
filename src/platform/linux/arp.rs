use std::fs;
use std::net::IpAddr;


use crate::sense::observation::{
    Attribute, IdentityHints, Observation, ResourceKind, SourceKind,
};

pub fn read_arp_table() -> std::io::Result<String>{
    fs::read_to_string("/proc/net/arp")
}


pub fn parse_arp_table(content: &str) -> Vec<Observation> {
    content
        .lines()
        .skip(1)
        .filter_map(|line| {
            let columns: Vec<&str> = line.split_whitespace().collect();

            if columns.len() < 6 {
                return None;
            }

            let ip: IpAddr = columns[0].parse().ok()?;
            let flags = columns[2];
            let mac = columns[3];
            let interface = columns[5];

            if flags != "0x2" || mac == "00:00:00:00:00:00" {
                return None;
            }

            Some(Observation {
                source: SourceKind::Arp,
                kind: ResourceKind::NetworkHost,
                identity: IdentityHints {
                    ip: Some(ip),
                    mac: Some(mac.to_string()),
                    hostname: None,
                },
                attributes: vec![Attribute::Interface(interface.to_string())],
            })
        })
        .collect()
}