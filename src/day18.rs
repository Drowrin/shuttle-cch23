use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    routing, Json, Router,
};
use serde::Deserialize;
use sqlx::PgPool;

async fn reset(State(pool): State<PgPool>) {
    sqlx::query("DROP TABLE IF EXISTS regions;")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DROP TABLE IF EXISTS orders;")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(
        "
        CREATE TABLE regions (
            id INT PRIMARY KEY,
            name VARCHAR(50)
        );          
        ",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "
        CREATE TABLE orders (
            id INT PRIMARY KEY,
            region_id INT,
            gift_name VARCHAR(50),
            quantity INT
        );
        ",
    )
    .execute(&pool)
    .await
    .unwrap();
}

#[derive(Deserialize)]
struct Region {
    id: i64,
    name: String,
}

async fn regions(State(pool): State<PgPool>, Json(regions): Json<Vec<Region>>) {}

async fn total(State(pool): State<PgPool>) -> Json<Vec<HashMap<String, i64>>> {}

async fn top_list(
    State(pool): State<PgPool>,
    Path(number): Path<i64>,
) -> Json<Vec<HashMap<String, i64>>> {
}

pub fn get_routes() -> Router<PgPool> {
    Router::new()
        .route("/18/reset", routing::post(reset))
        .route("/18/orders", routing::post(crate::day13::orders))
        .route("/18/regions", routing::post(regions))
        .route("/18/regions/total", routing::get(total))
        .route("/18/regions/top_list/:number", routing::get(top_list))
}
