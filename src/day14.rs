use askama::Template;
use axum::{routing, Json, Router};
use serde::Deserialize;

#[derive(Deserialize, Template)]
#[template(path = "day14.html", escape = "none")]
struct UnsafeTemplate {
    content: String,
}

async fn unsafe_html(Json(p): Json<UnsafeTemplate>) -> String {
    p.render().unwrap()
}

#[derive(Deserialize, Template)]
#[template(path = "day14.html")]
struct SafeTemplate {
    content: String,
}

async fn safe_html(Json(p): Json<SafeTemplate>) -> String {
    p.render().unwrap()
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/14/unsafe", routing::post(unsafe_html))
        .route("/14/safe", routing::post(safe_html))
}
