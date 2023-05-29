use std::{collections::HashMap, net::SocketAddr};

use log::info;
use warp::Filter;

pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub async fn run(&self) {
        let bind_endpoint = SocketAddr::from(([127, 0, 0, 1], 8080));
        info!("Listening on {}", bind_endpoint);

        let prompt = warp::get()
            .and(warp::path("hello"))
            .and(warp::path::param())
            .map(|name: String| format!("Hello, {}!", name))
            .or(warp::post()
                .and(warp::path("data"))
                .and(warp::body::json())
                .and_then(|data: HashMap<String, String>| async move {
                    Ok::<_, warp::Rejection>(warp::reply::json(&data))
                }));

        warp::serve(prompt).run(bind_endpoint).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::http::StatusCode;

    #[tokio::test]
    async fn hello_world() {
        let server = Server::new();
        let server_task = tokio::spawn(async move {
            server.run().await;
        });

        let response = reqwest::get("http://localhost:8080/hello/world")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        assert_eq!(response, "Hello, world!");
    }
}
