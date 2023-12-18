use axum::{extract::Path, http::StatusCode, routing::get, Router};
use sqlx::PgPool;
use std::{num::ParseIntError, ops::BitXor};

async fn cube_bits(Path(path): Path<String>) -> Result<String, (StatusCode, String)> {
    Ok(path
        .split("/")
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<i64>, ParseIntError>>()
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("path params should be ints: {e}"),
            )
        })?
        .into_iter()
        .reduce(i64::bitxor)
        .ok_or((StatusCode::BAD_REQUEST, "need at least one int".to_string()))?
        .pow(3)
        .to_string())
}

pub fn get_routes() -> Router<PgPool> {
    Router::new().route("/1/*path", get(cube_bits))
}
