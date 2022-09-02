mod router;

use axum::{routing::get, Router};
use router::*;

#[tokio::main]
async fn main() {
    env_logger::init();
    println!("Server started.");

    let app = Router::new().route("/", get(health));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
