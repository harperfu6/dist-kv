mod kvstore;
mod server;

use log::LevelFilter;
use server::Server;

static LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= LevelFilter::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(LevelFilter::Info);

    start_server().await;
    Ok(())
}

async fn start_server() {
    let server = Server::new();
    server.run().await;
}
