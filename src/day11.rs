use axum::{extract::Multipart, routing, Router};
use image::GenericImageView;
use tower_http::services::ServeDir;

async fn red_pixels(mut multipart: Multipart) -> String {
    image::load_from_memory(
        &multipart
            .next_field()
            .await
            .unwrap()
            .unwrap()
            .bytes()
            .await
            .unwrap(),
    )
    .unwrap()
    .pixels()
    .map(|(_, _, p)| (p.0[0], p.0[1], p.0[2]))
    .filter(|(r, g, b)| *r > g.saturating_add(*b))
    .count()
    .to_string()
}

pub fn get_routes() -> Router {
    Router::new()
        .nest_service("/11/assets", ServeDir::new("assets"))
        .route("/11/red_pixels", routing::post(red_pixels))
}
