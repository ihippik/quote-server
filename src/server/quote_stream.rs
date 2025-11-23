use std::net::UdpSocket;
use std::sync::mpsc::{Receiver};
use crate::server::stock::StockQuote;
use tracing::{debug, error, info};
use std::time::{Duration, Instant};
use std::io::ErrorKind;
use std::sync::mpsc::TryRecvError;

pub struct QuoteStream {
    socket: UdpSocket,
}

impl QuoteStream {
    pub fn new() ->  Result<Self, std::io::Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        Ok(Self { socket })
    }

    pub fn stream_start(
        &mut self,
        addr: &str,
        tickets: Vec<String>,
        rx_stock: Receiver<StockQuote>,
    ) {
        info!("streaming quotes for {}...addr:{}", tickets.join(","), addr);

        if let Err(e) = self.socket.send_to(b"hello", addr) {
            error!("failed to send UDP packet to {}: {}", addr, e);
        }

        let ping_timeout = Duration::from_secs(10);
        let mut last_ping = Instant::now();

        if let Err(e) = self.socket
            .set_read_timeout(Some(Duration::from_millis(500))){
            error!("failed to set read timeout: {}", e);
        }

        let mut buf = [0u8; 64];

        loop {
            if last_ping.elapsed() > ping_timeout {
                info!("ping timeout, closing stream addr:{}", addr);
                break;
            }

            match rx_stock.try_recv() {
                Ok(msg) => {
                    if !tickets.contains(&msg.ticker) {
                        debug!(ticker = msg.ticker, "ticker not in the list of requested stocks");
                    } else {
                        match self.socket.send_to(msg.to_bytes().as_slice(), addr) {
                            Ok(_) => {
                                debug!(addr = addr, ticker = msg.ticker, "message was sent to the client");
                            }
                            Err(e) => {
                                error!(%e, "failed to send message to the client");
                                break;
                            }
                        }
                    }
                }
                Err(TryRecvError::Empty) => {
                }
                Err(TryRecvError::Disconnected) => {
                    info!("quotes channel closed, stopping stream addr:{}", addr);
                    break;
                }
            }

            match self.socket.recv_from(&mut buf) {
                Ok((n, src)) => {
                    let msg = &buf[..n];

                    if msg == b"PING" {
                        debug!(?src, "ping received");
                        last_ping = Instant::now();
                    } else {
                        debug!(?src, msg = ?String::from_utf8_lossy(msg), "non-ping udp packet received");
                    }
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut => {
                }
                Err(e) => {
                    error!(%e, "error receiving ping on udp socket");
                    break;
                }
            }
        }

        debug!("udp streaming closed addr:{}", addr);
    }
}