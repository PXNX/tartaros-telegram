use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, Wordghhhhld!" }));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    let address = SocketAddr::from(([0, 0, 0, 0], port));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
