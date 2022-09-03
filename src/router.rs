use axum::{debug_handler, extract::Json, extract::Path, response::Redirect, Extension};
use log::info;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Payload {
    url: String,
}

#[debug_handler]
pub async fn health() -> String {
    "Hello, developer.".to_string()
}

#[debug_handler]
pub async fn shorten(Json(payload): Json<Payload>, Extension(db): Extension<sled::Db>) -> String {
    info!("{:?}", payload);
    let uuid = nanoid::nanoid!(8);
    let url_as_bytes = payload.url.as_bytes();
    db.insert(&uuid, url_as_bytes).unwrap();
    info!("key: {}, value: {:?}", uuid, url_as_bytes);
    assert_eq!(&db.get(uuid.as_bytes()).unwrap().unwrap(), url_as_bytes);
    uuid
}

#[debug_handler]
pub async fn redirect(Path(id): Path<String>, Extension(db): Extension<sled::Db>) -> Redirect {
    match &db.get(&id).unwrap() {
        Some(url) => {
            let url = String::from_utf8(url.to_vec()).unwrap();
            info!("URL found: {:#?}", url);
            Redirect::to(&url)
        }
        None => {
            info!("URL not found.");
            Redirect::to("/")
        }
    }
}
