use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use log::info;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use warp::{path, Filter};

use crate::kvstore::KvStore;

use jsonwebtoken::{DecodingKey, Validation};

pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub async fn run(&self, store: Arc<KvStore>) {
        let bind_endpoint = SocketAddr::from(([127, 0, 0, 1], 8080));
        info!("Listening on {}", bind_endpoint);

        warp::serve(route_filter(store)).run(bind_endpoint).await;
    }
}

fn route_filter(
    store: Arc<KvStore>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let store = warp::any().map(move || store.clone());

    let auth = warp::header::optional::<String>("auth")
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

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

async fn verify_auth(auth_header: Option<String>) -> Result<(), warp::Rejection> {
    let my_claims = Claims {
        sub: "".to_string(),
        company: "".to_string(),
        exp: 10000000000,
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &my_claims,
        &jsonwebtoken::EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();

    match auth_header {
        Some(auth_header) => {
            match jsonwebtoken::decode::<Claims>(
                // auth_header.trim_start_matches("Bearer "),
                &token,
                &DecodingKey::from_secret("secret".as_ref()),
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

    #[tokio::test]
    async fn server_test_all() {
        server_run().await;
        health().await;
        put_and_get_data().await;
    }

    async fn server_run() {
        let server = Server::new();
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

    async fn put_and_get_data() {
        let client = reqwest::Client::new();
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
