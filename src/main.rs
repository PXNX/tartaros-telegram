use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    Json,
    response::IntoResponse,
    Router, routing::{get, post},
};
use serde::{Deserialize, Serialize};

use tartaros_telegram::{establish_connection, report_user};

#[tokio::main]
async fn main() {
    let conn = establish_connection();

    let app = Router::new()
        .route("/", get(root))
        .route("/", post(post_user(&conn)));

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

async fn root() -> &'static str {
    "Hello, World!"
}



