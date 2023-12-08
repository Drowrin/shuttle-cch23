use axum::{extract::Path, http::StatusCode, routing, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pokemon {
    weight: f64,
}

async fn get_weight(id: u32) -> Result<f64, (StatusCode, String)> {
    let pokemon: Pokemon = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{id}/"))
        .await
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                format!("could not find pokemon {id}"),
            )
        })?
        .json::<Pokemon>()
        .await
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                format!("could not find pokemon {id}"),
            )
        })?;

    Ok(pokemon.weight / 10.0)
}

async fn weight(Path(id): Path<u32>) -> Result<String, (StatusCode, String)> {
    Ok(get_weight(id).await?.to_string())
}

const DROP: f64 = 2.0 * 9.825 * 10.0;
async fn drop(Path(id): Path<u32>) -> Result<String, (StatusCode, String)> {
    Ok((get_weight(id).await? * DROP.sqrt()).to_string())
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/8/weight/:id", routing::get(weight))
        .route("/8/drop/:id", routing::get(drop))
}
