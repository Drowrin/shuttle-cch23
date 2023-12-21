use axum::{extract::Query, http::StatusCode, routing, Json, Router};
use serde::Deserialize;
use serde_json::Value;
use sqlx::PgPool;

#[derive(Deserialize, Debug)]
struct Params {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}

async fn slice_names(
    Query(params): Query<Params>,
    Json(names): Json<Vec<String>>,
) -> Result<Json<Value>, StatusCode> {
    let offset = params.offset.unwrap_or(0);

    let names: Vec<Value> = match params.limit {
        Some(limit) if offset + limit >= names.len() => names.get(offset..),
        Some(limit) => names.get(offset..offset + limit),
        None => names.get(offset..),
    }
    .ok_or(StatusCode::BAD_REQUEST)?
    .iter()
    .map(|s| Value::String(s.to_string()))
    .collect();

    if let Some(split) = params.split {
        return Ok(Json(Value::Array(
            names
                .chunks(split)
                .map(|v| Value::Array(v.to_vec()))
                .collect::<Vec<_>>(),
        )));
    }

    Ok(Json(Value::Array(names)))
}

pub fn get_routes() -> Router<PgPool> {
    Router::new().route("/5", routing::post(slice_names))
}
