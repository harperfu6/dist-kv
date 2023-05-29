mod kvstore;
mod server;

use log::LevelFilter;
use server::Server;
use snafu::prelude::*;

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
async fn main() -> Result<(), Error> {
    log::set_logger(&LOGGER).context(LoggerSnafu)?;
    log::set_max_level(LevelFilter::Info);

    start_server().await;

    Ok(())
}

async fn start_server() {
    let server = Server::new();
    server.run().await;
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to set logger: {}", source))]
    Logger { source: log::SetLoggerError },
}
