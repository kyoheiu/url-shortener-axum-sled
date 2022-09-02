mod router;

use axum::{
    routing::{get, post},
    Router,
};
use router::*;

#[tokio::main]
async fn main() {
    env_logger::init();
    println!("Server started.");

    let app = Router::new()
        .route("/", get(health))
        .route("/shorten", post(shorten));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
