use axum::{extract::Path, http::StatusCode, routing::get, Router};
use std::{num::ParseIntError, ops::BitXor};

async fn cube_bits(Path(path): Path<String>) -> Result<String, (StatusCode, String)> {
    Ok(path
        .split("/")
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("path params should be ints: {e}"),
            )
        })?
        .into_iter()
        .reduce(u32::bitxor)
        .ok_or((StatusCode::BAD_REQUEST, "need at least one int".to_string()))?
        .pow(3)
        .to_string())
}

pub fn get_routes() -> Router {
    Router::new().route("/1/*path", get(cube_bits))
}
