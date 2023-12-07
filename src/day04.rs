use axum::{http::StatusCode, routing, Json, Router};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Reindeer {
    strength: u32,
    // validate name even though we don't use it
    // that way our API remains stable if we need it in the future
    #[serde(rename = "name")]
    _name: String,
}

async fn strength(Json(herd): Json<Vec<Reindeer>>) -> String {
    herd.iter()
        .map(|reindeer| reindeer.strength)
        .sum::<u32>()
        .to_string()
}

#[derive(Deserialize)]
struct ContestReindeer {
    name: String,
    strength: u32,
    speed: f32,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    consumed: u32,
}

#[derive(Serialize)]
struct Summary {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

fn winners<'s>(
    herd: &'s Vec<ContestReindeer>,
) -> Option<(
    &'s ContestReindeer,
    &'s ContestReindeer,
    &'s ContestReindeer,
    &'s ContestReindeer,
)> {
    Some((
        herd.iter()
            .max_by_key(|reindeer| OrderedFloat(reindeer.speed))?,
        herd.iter().max_by_key(|reindeer| reindeer.height)?,
        herd.iter()
            .max_by_key(|reindeer| reindeer.snow_magic_power)?,
        herd.iter().max_by_key(|reindeer| reindeer.consumed)?,
    ))
}

impl Summary {
    fn new(herd: &Vec<ContestReindeer>) -> Result<Self, (StatusCode, String)> {
        let (fastest, tallest, magician, consumer) = winners(herd).ok_or((
            StatusCode::BAD_REQUEST,
            "need at least one reindeer".to_string(),
        ))?;

        Ok(Summary {
            fastest: format!(
                "Speeding past the finish line with a strength of {strength} is {name}",
                strength = fastest.strength,
                name = fastest.name
            ),
            tallest: format!(
                "{name} is standing tall with his {antler_width} cm wide antlers",
                name = tallest.name,
                antler_width = tallest.antler_width,
            ),
            magician: format!(
                "{name} could blast you away with a snow magic power of {snow_magic_power}",
                name = magician.name,
                snow_magic_power = magician.snow_magic_power,
            ),
            consumer: format!(
                "{name} ate lots of candies, but also some {favorite_food}",
                name = consumer.name,
                favorite_food = consumer.favorite_food
            ),
        })
    }
}

async fn contest(
    Json(herd): Json<Vec<ContestReindeer>>,
) -> Result<Json<Summary>, (StatusCode, String)> {
    Ok(Json(Summary::new(&herd)?))
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/4/strength", routing::post(strength))
        .route("/4/contest", routing::post(contest))
}
