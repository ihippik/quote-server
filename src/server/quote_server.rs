use std::net::UdpSocket;
use tracing::{info};

pub struct QuoteServer {
    socket: UdpSocket,
}

impl QuoteServer {
    pub fn new(bind_addr: &str) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(bind_addr)?;

        info!(addr = bind_addr, "server started");

        Ok(Self { socket })
    }
}