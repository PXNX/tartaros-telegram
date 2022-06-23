use std::net::SocketAddr;

use axum::{Router, routing::get};

use tartaros_telegram::{establish_connection, report_user};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    let address = SocketAddr::from(([0, 0, 0, 0], port));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    let conn = establish_connection();
    report_user(&conn, &12345, "test");
}
