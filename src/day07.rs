use std::collections::HashMap;

use axum::{http::StatusCode, routing, Json, Router};
use axum_extra::extract::cookie::CookieJar;
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use serde_json;

fn decode(encoded: &str) -> Result<Vec<u8>, (StatusCode, String)> {
    general_purpose::STANDARD.decode(encoded).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("recipe cookie improperly encoded: {e}"),
        )
    })
}

fn get_recipe(jar: CookieJar) -> Result<Vec<u8>, (StatusCode, String)> {
    decode(
        jar.get("recipe")
            .ok_or((StatusCode::BAD_REQUEST, "recipe cookie missing".to_string()))?
            .value(),
    )
}

async fn decode_endpoint(jar: CookieJar) -> Result<Vec<u8>, (StatusCode, String)> {
    get_recipe(jar)
}

#[derive(Deserialize)]
struct BakeRequest {
    recipe: HashMap<String, u32>,
    pantry: HashMap<String, u32>,
}

#[derive(Serialize)]
struct BakeResponse {
    cookies: u32,
    pantry: HashMap<String, u32>,
}

async fn bake(jar: CookieJar) -> Result<Json<BakeResponse>, (StatusCode, String)> {
    let BakeRequest { pantry, recipe } = serde_json::from_slice(&get_recipe(jar)?)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("json encoding error: {e}")))?;

    let cookies = recipe
        .iter()
        .map(|(k, v)| pantry.get(k).unwrap_or(&0) / v)
        .min()
        .ok_or((
            StatusCode::BAD_REQUEST,
            "recipe must contain some ingredients".to_string(),
        ))?;

    Ok(Json(BakeResponse {
        cookies,
        pantry: pantry
            .into_iter()
            .map(|(k, v)| (k.clone(), v - cookies * recipe.get(&k).unwrap_or(&0)))
            .collect(),
    }))
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/7/decode", routing::get(decode_endpoint))
        .route("/7/bake", routing::get(bake))
}
