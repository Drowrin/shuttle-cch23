use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::{Instant, SystemTime},
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing, Json, Router,
};
use chrono::{DateTime, Datelike, Utc};
use itertools::Itertools;
use sqlx::PgPool;
use ulid::Ulid;
use uuid::Uuid;

type SharedState = Arc<RwLock<HashMap<String, Instant>>>;

async fn save(State(state): State<SharedState>, Path(packet_id): Path<String>) {
    state.write().unwrap().insert(packet_id, Instant::now());
}

async fn load(
    State(state): State<SharedState>,
    Path(packet_id): Path<String>,
) -> Result<String, StatusCode> {
    Ok(state
        .read()
        .unwrap()
        .get(&packet_id)
        .ok_or(StatusCode::NOT_FOUND)?
        .elapsed()
        .as_secs()
        .to_string())
}

async fn ulids(Json(data): Json<Vec<String>>) -> Json<Vec<String>> {
    Json(
        data.into_iter()
            .rev()
            .map(|s| Uuid::from_u128(Ulid::from_string(&s).unwrap().0).to_string())
            .collect(),
    )
}

async fn weekday(
    Path(day): Path<String>,
    Json(data): Json<Vec<String>>,
) -> Json<HashMap<String, usize>> {
    let ulids = data
        .iter()
        .map(|s| Ulid::from_string(&s).unwrap())
        .collect_vec();

    let dates = ulids
        .iter()
        .map(|ulid| DateTime::<Utc>::from(ulid.datetime()))
        .collect_vec();

    Json(HashMap::from([
        (
            "christmas eve".into(),
            dates
                .iter()
                .filter(|date| date.month() == 12 && date.day() == 24)
                .count(),
        ),
        (
            "weekday".into(),
            dates
                .iter()
                .filter(|date| date.weekday().num_days_from_monday() == day.parse::<u32>().unwrap())
                .count(),
        ),
        (
            "in the future".into(),
            dates
                .iter()
                .filter(|date| **date > DateTime::<Utc>::from(SystemTime::now()))
                .count(),
        ),
        (
            "LSB is 1".into(),
            ulids.iter().filter(|ulid| ulid.0 & 1 == 1).count(),
        ),
    ]))
}

pub fn get_routes() -> Router<PgPool> {
    Router::new()
        .route("/12/save/:packet_id", routing::post(save))
        .route("/12/load/:packet_id", routing::get(load))
        .route("/12/ulids", routing::post(ulids))
        .route("/12/ulids/:weekday", routing::post(weekday))
        .with_state(Arc::new(RwLock::new(HashMap::<String, Instant>::new())))
}
