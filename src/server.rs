use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use chrono::{DateTime, Utc};
use log::info;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use warp::{path, Filter};

use crate::{
    configuration::{self, Claims, Configuration},
    kvstore::KvStore,
};

use jsonwebtoken::{DecodingKey, Header, Validation};

pub struct Server {
    configuration: Configuration,
}

impl Server {
    pub fn new(configuration: Configuration) -> Server {
        Server { configuration }
    }

    pub async fn run(&self, store: Arc<KvStore>) {
        let bind_endpoint = SocketAddr::from(([127, 0, 0, 1], 8080));
        info!("Listening on {}", bind_endpoint);

        warp::serve(route_filter(store, Arc::new(self.configuration.clone())))
            .run(bind_endpoint)
            .await;
    }
}

fn route_filter(
    store: Arc<KvStore>,
    configuration: Arc<Configuration>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let store = warp::any().map(move || store.clone());
    let config = warp::any().map(move || configuration.clone());

    let auth = warp::header::optional::<String>("auth")
        .and(config)
        .and_then(verify_auth)
        .untuple_one();

    let api_kv = warp::get()
        .and(store.clone())
        .and(path!("api" / "kv" / String))
        .and_then(get_data)
        .or(warp::post()
            .and(store.clone())
            .and(path!("api" / "kv"))
            .and(warp::body::json::<HashMap<String, String>>())
            .and_then(post_data));

    let health = warp::path("health").map(|| Ok(warp::reply::with_status("OK", StatusCode::OK)));

    auth.and(api_kv).or(health)
}

async fn verify_auth(
    auth_header: Option<String>,
    config: Arc<Configuration>,
) -> Result<(), warp::Rejection> {
    if config.authentication.enabled {
        match auth_header {
            Some(auth_header) => {
                match jsonwebtoken::decode::<Claims>(
                    auth_header.trim_start_matches("Bearer "), // token,
                    &DecodingKey::from_secret(config.authentication.secret_key.as_ref()),
                    &Validation::default(),
                ) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        println!("error by: {:?}", e);
                        Err(warp::reject::custom(Error::InvalidJwt))
                    }
                }
            }
            None => Err(warp::reject::custom(Error::MissingAuthHeader)),
        }
    } else {
        Ok(())
    }
}

async fn get_data(store: Arc<KvStore>, key: String) -> Result<impl warp::Reply, warp::Rejection> {
    info!("GET by: {}", key);
    match store.get(key) {
        Some(v) => Ok(warp::reply::with_status(v, StatusCode::OK)),
        None => Ok(warp::reply::with_status(
            "The specified key does not exist.".to_string(),
            StatusCode::NOT_FOUND,
        )),
    }
}

async fn post_data(
    store: Arc<KvStore>,
    data: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("POST DATA: {:?}", data);
    store.set(data);
    Ok(warp::reply::with_status(
        "The specified key was successfully created.".to_string(),
        StatusCode::CREATED,
    ))
}

#[derive(Debug)]
enum Error {
    MissingAuthHeader,
    InvalidJwt,
}

impl warp::reject::Reject for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{generate_secret_key, issue_jwt};
    use configuration::Configuration;
    use lazy_static::lazy_static;

    lazy_static! {
        // for without authentication
        static ref CONFIG: Configuration = Configuration::default();

        // for with authentication
        static ref SECRET_KEY: String = generate_secret_key();
        static ref JWT: String = issue_jwt(&SECRET_KEY, None).unwrap();
        static ref CONFIG_WITH_AUTHENTICATION: Configuration = Configuration::new_with_enabled(JWT.clone(), SECRET_KEY.clone());
    }

    // #[tokio::test]
    // async fn server_test_without_authentication() {
    //     server_run().await;
    //     health().await;
    //     post_and_get_data().await;
    // }

    #[tokio::test]
    async fn server_test_with_authentiation() {
        server_run_with_authentication().await;
        post_and_get_data_with_authentication().await;
    }

    async fn server_run() {
        let server = Server::new(CONFIG.clone());
        let store = Arc::new(KvStore::new());
        let _ = tokio::spawn(async move {
            server.run(store).await;
        });
    }

    async fn server_run_with_authentication() {
        let server = Server::new(CONFIG_WITH_AUTHENTICATION.clone());
        let store = Arc::new(KvStore::new());
        let _ = tokio::spawn(async move {
            server.run(store).await;
        });
    }

    async fn health() {
        let response = reqwest::get("http://localhost:8080/health")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        assert_eq!(response, "OK");
    }

    async fn post_and_get_data() {
        let client = reqwest::Client::builder().build().unwrap();

        let _ = client
            .post("http://localhost:8080/api/kv")
            .json(&HashMap::from([("key".to_string(), "value".to_string())]))
            .send()
            .await
            .unwrap();

        let response = client
            .get("http://localhost:8080/api/kv/key")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        assert_eq!(response, "value");
    }

    async fn post_and_get_data_with_authentication() {
        let mut headers = reqwest::header::HeaderMap::new();
        let bearer = format!(
            "Bearer {}",
            CONFIG_WITH_AUTHENTICATION.authentication.root_token
        );
        headers.insert("auth", bearer.parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let _ = client
            .post("http://localhost:8080/api/kv")
            .json(&HashMap::from([("key".to_string(), "value".to_string())]))
            .send()
            .await
            .unwrap();

        let response = client
            .get("http://localhost:8080/api/kv/key")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        assert_eq!(response, "value");
    }
}
