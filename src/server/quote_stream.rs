use std::net::UdpSocket;
use std::sync::mpsc::{Receiver};
use crate::server::stock::StockQuote;
use tracing::{debug, error, info};

pub struct QuoteStream {
    socket: UdpSocket,
}

impl QuoteStream {
    pub fn new() ->  Result<Self, std::io::Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        Ok(Self { socket })
    }

    pub fn stream_start(&mut self, addr: &str, tickets: Vec<String>, rx_stock: Receiver<StockQuote>) {
        info!("streaming quotes for {}...addr:{}", tickets.join(","),addr);

        for msg in rx_stock {
            if !tickets.contains(&msg.ticker) {
                debug!(ticker = msg.ticker, "ticker not in the list of requested stocks");
                continue;
            }

            match self.socket.send_to(msg.to_bytes().as_slice(), addr){
                Ok(_) => {
                    debug!(addr=addr,ticker=msg.ticker, "message was sent to the client");
                }
                Err(e) => {
                    error!(%e, "failed to send message to the client");
                    return;
                }
            }
        }
    }
}