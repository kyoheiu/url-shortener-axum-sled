mod router;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use log::info;
use router::*;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Server started.");

    let db: sled::Db = sled::open("my_db").unwrap();
    info!("Database started.");

    let app = Router::new()
        .route("/", get(health))
        .route("/shorten", post(shorten))
        .layer(Extension(db));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
