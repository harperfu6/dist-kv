use std::net::SocketAddr;

use log::info;

pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub async fn run(&self) {
        let bind_endpoint = SocketAddr::from(([127, 0, 0, 1], 8080));
        info!("Listening on {}", bind_endpoint);
    }
}
