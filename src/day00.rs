use axum::{http::StatusCode, routing::get, Router};

async fn root() -> () {}

async fn fake_error() -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, "Fake error!".to_string())
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/-1/error", get(fake_error))
}
