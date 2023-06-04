mod configuration;
mod kvstore;
mod server;

use std::{fmt, fs, path::Path, sync::Arc};

use chrono::{DateTime, Duration, Utc};
use configuration::{Claims, Configuration};
use jsonwebtoken::{EncodingKey, Header};
use kvstore::KvStore;
use log::{error, info, LevelFilter};
use rand::Rng;
use ring::digest;
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

    let config_path = Path::new("config.yaml");
    // TODO: Check if config file exists and if so, read it and use the values
    let config = Configuration::default();
    if let Err(e) = start_server(config, &config_path).await {
        error!("Error: {}", e);
    }

    Ok(())
}

async fn start_server(config: Configuration, config_path: &Path) -> Result<(), Error> {
    let mut config = Configuration::default();
    let secret_key = generate_secret_key();
    let jwt = issue_jwt(&secret_key, None)?;

    config.authentication.root_token = jwt;
    config.authentication.secret_key = secret_key;

    fs::create_dir_all(config_path.parent().unwrap()).context(CreateConfigDirSnafu)?;
    serde_yaml::to_writer(
        fs::File::create(config_path).context(CreateConfigFileSnafu)?,
        &config,
    )
    .context(WriteConfigFileSnafu)?;

    let server = Server::new(config);
    let store = Arc::new(KvStore::new());
    server.run(store).await;

    Ok(())
}

fn generate_secret_key() -> String {
    let secret_key_bytes = digest::digest(&digest::SHA256, &rand::thread_rng().gen::<[u8; 32]>());
    secret_key_bytes.as_ref().iter().fold(
        String::with_capacity(secret_key_bytes.as_ref().len() * 2),
        |mut acc, x| {
            acc.push_str(&format!("{:0>2x}", x));
            acc
        },
    )
}

fn issue_jwt(secret_key: &str, expiration: Option<DateTime<Utc>>) -> Result<String, Error> {
    jsonwebtoken::encode(
        &Header::default(),
        &Claims {
            sub: String::from("user"),
            iss: String::from("issuer"),
            iat: Utc::now().timestamp(),
            exp: match expiration {
                Some(exp) => exp.timestamp(),
                None => (Utc::now() + Duration::weeks(52 * 3)).timestamp(), // 3 years
            },
        },
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .context(EncodeJwtSnafu)
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to set logger: {}", source))]
    Logger { source: log::SetLoggerError },
    #[snafu(display("Failed to issue JWT: {}", source))]
    EncodeJwt { source: jsonwebtoken::errors::Error },
    #[snafu(display("Failed to create config directory: {}", source))]
    CreateConfigDir { source: std::io::Error },
    #[snafu(display("Failed to create config file: {}", source))]
    CreateConfigFile { source: std::io::Error },
    #[snafu(display("Failed to write config file: {}", source))]
    WriteConfigFile { source: serde_yaml::Error },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secret_key() {
        let secret_key = generate_secret_key();
        assert_eq!(secret_key.len(), 64);
    }

    #[test]
    fn test_issue_jwt() {
        let secret_key = generate_secret_key();
        let jwt = issue_jwt(&secret_key, None).unwrap();
        assert!(jwt.len() > 0);
    }
}
