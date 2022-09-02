use axum::debug_handler;

#[debug_handler]
pub async fn health() -> String {
    "Hello, developer.".to_string()
}
