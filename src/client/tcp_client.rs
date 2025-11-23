use std::net::TcpStream;
use std::io::{Write};

/// Simple TCP client wrapper.
pub struct TcpClient{
    stream: TcpStream
}

/// Errors returned by the TCP client.
#[derive(Debug)]
pub enum CliError{
    /// I/O operation failed.
    IOError
}

impl TcpClient {
    /// Creates a new TCP client and connects to the given address.
    pub fn new(addr: &str) -> Self{
        let stream = TcpStream::connect(addr).expect("failed to connect");
        TcpClient{stream}
    }

    /// Sends a command over TCP.
    pub fn send_command(&mut self, msg: &str) -> Result<(), CliError> {
        self.stream.write_all(msg.as_bytes()).map_err(|_| CliError::IOError)?;
        self.stream.flush().map_err(|_| CliError::IOError)?;
        Ok(())
    }
}