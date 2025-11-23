use clap::Parser;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tracing::{info, error};
use quote_server::client::tcp_client::TcpClient;
use quote_server::client::udp_server::UdpServer;

#[derive(Parser, Debug)]
#[command(name="client", version, about="quote stream client")]
struct Cli {
    #[arg(long="address")]
    address: String,

    #[arg(long="udp_port")]
    udp_port: u16,

    #[arg(long="tickets")]
    tickets: String,
}

fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let udp_server = match UdpServer::new(cli.udp_port) {
        Ok(srv) => srv,
        Err(e) => {
            error!("failed to create UdpServer: {}", e);
            return;
        }
    };

    let mut tcp_client = TcpClient::new(cli.address.as_ref());
    info!("connected to tcp server: {}", cli.address);

    let command = format!("STREAM 127.0.0.1:{} {}", cli.udp_port, cli.tickets);
    if let Err(err) = tcp_client.send_command(&command) {
        error!("failed to send command: {:?}", err);
        return;
    }
    info!("sent command: {}", command);

    let stop = Arc::new(AtomicBool::new(false));
    {
        let stop = stop.clone();
        ctrlc::set_handler(move || {
            stop.store(true, Ordering::SeqCst);
            error!("ctrl+c caught, shutting down...");
        }).expect("error setting Ctrl+C handler");
    }

    let ping_handle = udp_server
        .start_ping_thread(stop.clone())
        .expect("failed to start ping thread");

    udp_server.recv_loop(&stop);

    info!("client shutting down...");

    let _ = ping_handle.join();
}
