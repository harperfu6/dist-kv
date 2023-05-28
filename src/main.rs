mod kvstore;
mod server;
use log::LevelFilter;
use server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    log::set_max_level(LevelFilter::Info);

    start_server().await;
    Ok(())
}

async fn start_server() {
    let server = Server::new();
    server.run().await;
}
