use std::net::UdpSocket;
use std::sync::mpsc::{Receiver,Sender};
use crate::server::stock::StockQuote;
use tracing::{debug};
use crate::server::generator::QuoteGenerator;

struct QuoteStream {
    socket: UdpSocket,
}

impl QuoteStream {
    pub fn new() ->  Result<Self, std::io::Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        Ok(Self { socket })
    }

    fn stream_start(&mut self, addr: &str, rx_stock: Receiver<StockQuote>) {
        for msg in rx_stock {
            self.socket.send_to(msg.to_bytes().as_slice(), addr).unwrap();
            debug!(addr=addr,"message was sent");
        }
    }
}