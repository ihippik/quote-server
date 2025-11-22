use tracing::{info, error};
use tracing_subscriber;
use quote_server::server::QuoteServer;

fn main(){
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:8084";

    match QuoteServer::new(addr){
        Ok(server) => {
            info!(addr,"server started");
            server.run();
        }
        Err(err) => {
            error!(%err, "failed to start server");
        }
    }
}