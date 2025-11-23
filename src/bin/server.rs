use clap::Parser;
use tracing::{info, error};
use tracing_subscriber;
use quote_server::server::QuoteServer;

#[derive(Parser, Debug)]
#[command(name="server", version, about="tcp server")]
struct Cli {
    #[arg(long="address")]
    address: String,
}

fn main(){
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match QuoteServer::new(cli.address.as_ref()){
        Ok(server) => {
            info!(cli.address,"server started");
            server.run();
        }
        Err(err) => {
            error!(%err, "failed to start server");
        }
    }
}