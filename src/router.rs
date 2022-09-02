use axum::{debug_handler, Extension};

#[debug_handler]
pub async fn health() -> String {
    "Hello, developer.".to_string()
}

#[debug_handler]
pub async fn shorten(body: String, Extension(db): Extension<sled::Db>) -> String {
    let uuid = nanoid::nanoid!(8);
    db.insert(uuid.as_bytes(), body.as_bytes()).unwrap();
    assert_eq!(&db.get(uuid.as_bytes()).unwrap().unwrap(), body.as_bytes());
    uuid
}
