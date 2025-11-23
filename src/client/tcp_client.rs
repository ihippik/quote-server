use std::net::TcpStream;
use std::io::{Write};

pub struct TcpClient{
    stream: TcpStream
}

#[derive(Debug)]
pub enum CliError{
    IOError
}

impl TcpClient {
    pub fn new(addr: &str) -> Self{
        let stream = TcpStream::connect(addr).expect("failed to connect");
        TcpClient{stream}
    }

    pub fn send_command(&mut self, msg: &str) -> Result<(), CliError> {
        self.stream.write_all(msg.as_bytes()).map_err(|_| CliError::IOError)?;
        self.stream.flush().map_err(|_| CliError::IOError)?;
        Ok(())
    }
}