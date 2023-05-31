use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use log::info;
use reqwest::StatusCode;
use warp::{path, reply::WithStatus, Filter};

use crate::kvstore::KvStore;

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

    api_kv.or(health)
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
