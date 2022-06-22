use crate::interface::Interface;
use std::{path::PathBuf};
use url::Url;

#[derive(Debug)]
pub enum Endpoint {    
    HostPort {
        host: Host,
        port: Port,
    },
    Service {
        name: String,
        type_: String,
        domain: String,
        interface: Option<Interface>,
    },
    Unix {
        path: PathBuf,
    },
    Url(Url),
    Opaque(crate::sys::nw_endpoint_t),
}

impl Endpoint {
    pub fn interface(&self) -> Option<Interface> {
        match self {
            Endpoint::HostPort {
                host: Host::Name(_, iface @ Some(_)),
                ..
            } => iface.clone(),
            Endpoint::Service {
                interface: iface @ Some(_),
                ..
            } => iface.clone(),
            _ => None, // Q: Could Opaque also return a Host
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Host {
    Name(String, Option<Interface>),
    IPv4(std::net::Ipv4Addr),
    IPv6(std::net::Ipv6Addr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Port {
    pub raw_value: u16,
}

impl Port {
    pub const fn new(raw_value: u16) -> Self {
        Self { raw_value }
    }

    pub const ANY: Self = Self::new(0);
    pub const SSH: Self = Self::new(22);
    pub const SMTP: Self = Self::new(25);
    pub const HTTP: Self = Self::new(80);
    pub const POP: Self = Self::new(110);
    pub const IMAP: Self = Self::new(143);
    pub const HTTPS: Self = Self::new(443);
    pub const IMAPS: Self = Self::new(993);
    pub const SOCKS: Self = Self::new(1080);
}

impl From<u16> for Port {
    fn from(raw_value: u16) -> Self {
        Self { raw_value }
    }
}
impl Into<u16> for Port {
    fn into(self) -> u16 {
        self.raw_value
    }
}
impl AsRef<u16> for Port {
    fn as_ref(&self) -> &u16 {
        &self.raw_value
    }
}
impl AsMut<u16> for Port {
    fn as_mut(&mut self) -> &mut u16 {
        &mut self.raw_value
    }
}
