use std::net::{SocketAddr, UdpSocket};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tracing::{debug, error, info};

pub struct UdpServer {
    socket: UdpSocket,
    server_addr: Arc<Mutex<Option<SocketAddr>>>,
}

impl UdpServer {
    pub fn new(port: u16) -> std::io::Result<Self> {
        let bind_addr = format!("0.0.0.0:{port}");
        let socket = UdpSocket::bind(&bind_addr)?;
        socket.set_read_timeout(Some(Duration::from_millis(500)))?;

        info!("UDP listening on {}", bind_addr);

        Ok(Self {
            socket,
            server_addr: Arc::new(Mutex::new(None)),
        })
    }

    pub fn start_ping_thread(
        &self,
        stop: Arc<AtomicBool>,
    ) -> std::io::Result<JoinHandle<()>> {
        let socket = self.socket.try_clone()?;
        let server_addr = Arc::clone(&self.server_addr);

        let handle = thread::spawn(move || {
            loop {
                if stop.load(Ordering::SeqCst) {
                    return;
                }

                let addr_opt = {
                    match server_addr.lock() {
                        Ok(guard) => *guard,
                        Err(poisoned) => {
                            error!("server_addr mutex poisoned: {}", poisoned);
                            continue;
                        }
                    }
                };

                if let Some(addr) = addr_opt {
                    debug!("ping thread started, server UDP addr: {}", addr);

                    while !stop.load(Ordering::SeqCst) {
                        if let Err(e) = socket.send_to(b"PING", addr) {
                            error!("failed to send ping: {}", e);
                            break;
                        }
                        debug!("ping sent");
                        thread::sleep(Duration::from_secs(2));
                    }

                    return;
                } else {
                    thread::sleep(Duration::from_millis(100));
                }
            }
        });

        Ok(handle)
    }

    pub fn recv_loop(&self, stop: &AtomicBool) {
        let mut buf = [0u8; 1500];

        while !stop.load(Ordering::SeqCst) {
            match self.socket.recv_from(&mut buf) {
                Ok((n, src)) => {
                    {
                        match self.server_addr.lock() {
                            Ok(mut guard) => {
                                if guard.is_none() {
                                    *guard = Some(src);
                                    debug!("first packet from server UDP addr: {}", src);
                                }
                            }
                            Err(poisoned) => {
                                error!("server_addr mutex poisoned: {}", poisoned);
                                continue;
                            }
                        }
                    }

                    let payload = &buf[..n];
                    info!("msg from {}: {}", src, String::from_utf8_lossy(payload));
                }
                Err(e)
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut =>
                    {
                        continue;
                    }
                Err(e) => {
                    error!("udp recv error: {}", e);
                    break;
                }
            }
        }

        info!("UDP recv loop finished");
    }
}