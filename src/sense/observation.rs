use std::net::IpAddr;

#[derive(Debug)]
pub enum SourceKind {
    Arp,
    Mdns,
    Ssdp,
    Tcp,
    Udev,
}

#[derive(Debug)]
pub enum ResourceKind {
    NetworkHost,
    Device,
    Service,
    Process,
    Unknown,
}

#[derive(Debug)]
pub struct IdentityHints {
    pub ip: Option<IpAddr>,
    pub mac: Option<String>,
    pub hostname: Option<String>,
}

#[derive(Debug)]
pub enum Attribute {
    TcpPort(u16),
    Hostname(String),
    Service(String),
    Vendor(String),
}

#[derive(Debug)]
pub struct Observation {
    pub source: SourceKind,
    pub kind: ResourceKind,
    pub identity: IdentityHints,
    pub attributes: Vec<Attribute>,
}