use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use log::info;
use warp::Filter;

use crate::kvstore::KvStore;

pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub async fn run(&self, store: Arc<KvStore>) {
        let bind_endpoint = SocketAddr::from(([127, 0, 0, 1], 8080));
        info!("Listening on {}", bind_endpoint);

        let store = warp::any().map(move || store.clone());

        let prompt = warp::get()
            .and(warp::path("hello"))
            .and(warp::path::param())
            .map(|name: String| format!("Hello, {}!", name))
            .or(warp::post()
                .and(store.clone())
                .and(warp::body::json::<HashMap<String, String>>())
                .and_then(post_data))
            .or(warp::get()
                .and(store.clone())
                .and(warp::path::param())
                .and_then(get_data));

        warp::serve(prompt).run(bind_endpoint).await;
    }
}

async fn get_data(store: Arc<KvStore>, key: String) -> Result<impl warp::Reply, warp::Rejection> {
    info!("GET by: {}", key);
    match store.get(key) {
        Some(v) => Ok::<_, warp::Rejection>(v),
        None => Ok::<_, warp::Rejection>("The specified key does not exist.".to_string()),
    }
}

async fn post_data(
    store: Arc<KvStore>,
    data: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("POST DATA: {:?}", data);
    store.set(data);
    Ok::<_, warp::Rejection>("The specified key was successfully created.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn server_test_all() {
        server_run().await;
        hello_world().await;
        put_and_get_data().await;
    }

    async fn server_run() {
        let server = Server::new();
        let store = Arc::new(KvStore::new());
        let _ = tokio::spawn(async move {
            server.run(store).await;
        });
    }

    async fn hello_world() {
        let response = reqwest::get("http://localhost:8080/hello/world")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        assert_eq!(response, "Hello, world!");
    }

    async fn put_and_get_data() {
        let client = reqwest::Client::new();
        let _ = client
            .post("http://localhost:8080")
            .json(&HashMap::from([("key".to_string(), "value".to_string())]))
            .send()
            .await
            .unwrap();

        let response = client
            .get("http://localhost:8080/key")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        assert_eq!(response, "value");
    }
}
