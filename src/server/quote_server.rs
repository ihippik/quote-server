use std::net::{TcpListener, TcpStream};
use tracing::{debug, error, info};
use std::io::{Read, Write};
use std::thread;
use crate::server::generator::QuoteGenerator;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver,Sender};
use crate::server::quote_stream::QuoteStream;
use crate::server::stock::StockQuote;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
enum ServerError {
    InvalidRequest,
}

const CMD_STREAM: &str = "STREAM";

pub struct QuoteServer {
    listener: TcpListener,
    subscribers: Arc<Mutex<Vec<Sender<StockQuote>>>>,
}

impl QuoteServer {
    pub fn new(addr: &str) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(addr)?;
        let subscribers = Arc::new(Mutex::new(Vec::new()));
        Ok(Self { listener,subscribers })
    }

    pub fn run(&self) {
        let mut generator = QuoteGenerator::new(vec![
            "MSFT".to_string()
        ]);
        generator.init();

        let subscribers = Arc::clone(&self.subscribers);
        thread::spawn(move ||generator.start(subscribers));

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Ok(peer) = stream.peer_addr() {
                        info!(%peer, "new incoming connection");
                    }

                    let (tx, rx) = mpsc::channel::<StockQuote>();

                    let mut guard = self.subscribers.lock().unwrap_or_else(|poisoned| {
                        error!("subscribers mutex poisoned in run(), recovering");
                        poisoned.into_inner()
                    });

                    guard.push(tx);
                    debug!("new subscriber added");

                    debug!("new subscriber added");

                    thread::spawn(|| Self::handle_client(stream,rx));
                }
                Err(e) =>  error!(%e, "failed to accept incoming connection")
            }
        }
    }

    fn handle_client(mut stream: TcpStream, rx: Receiver<StockQuote> ) {
        let mut buffer = [0u8; 512];

        loop {
            match stream.read(&mut buffer) {
                Ok(0) => {
                    info!("client disconnected");
                    break;
                }
                Ok(n) => {
                    let msg = String::from_utf8_lossy(&buffer[..n]).to_string();
                    let msg = msg.trim();

                    info!("received: `{}`", msg);

                    if let Ok(parts) = Self::parse_request(msg) {
                        if parts[0] == CMD_STREAM {
                            let addr = parts[1].clone();
                            let tickets_raw = parts[2].clone();

                            debug!(addr=parts[1],tickets=tickets_raw,"stream requested");

                            let tickets: Vec<String> = tickets_raw.split(',')
                                .map(|s| s.to_string())
                                .collect();

                            thread::spawn(move || {
                                match QuoteStream::new() {
                                    Ok(mut qs) => {
                                        qs.stream_start(&addr, tickets, rx);
                                    }
                                    Err(e) => {
                                        error!("failed to create QuoteStream: {}", e);
                                    }
                                }
                            });

                            debug!(addr=parts[1],"tcp stream closed");

                            break // close connection
                        }
                    }

                    if let Err(e) = stream.write_all("ok".as_bytes()) {
                        error!("failed to tcp send response: {}", e);
                    }
                }
                Err(e) => {
                    error!("error reading stream: {}", e);
                    break;
                }
            }
        }
    }

    fn parse_request(request: &str) -> Result<Vec<String>, ServerError> {
        let parts: Vec<String> = request.split_whitespace().map(|s| s.to_string())
            .collect();

        if parts.len() != 3 {
            return Err(ServerError::InvalidRequest);
        }

        Ok(parts)
    }
}