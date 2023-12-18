use axum::{
    extract::{Path, State},
    routing, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};

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

async fn regions(State(pool): State<PgPool>, Json(regions): Json<Vec<Region>>) {
    if regions.len() == 0 {
        return;
    }
    sqlx::QueryBuilder::new("INSERT INTO regions (id, name) ")
        .push_values(regions, |mut builder, region| {
            builder.push_bind(region.id).push_bind(region.name);
        })
        .build()
        .execute(&pool)
        .await
        .unwrap();
}

#[derive(Serialize, FromRow)]
struct Total {
    region: String,
    total: i64,
}

async fn total(State(pool): State<PgPool>) -> Json<Vec<Total>> {
    Json(
        sqlx::query_as(
            "
            SELECT
                r.name as region, SUM(o.quantity) as total
            FROM
                orders o
            JOIN regions r ON
                o.region_id = r.id
            GROUP BY
                r.name
            ORDER BY
                r.name
            ",
        )
        .fetch_all(&pool)
        .await
        .unwrap(),
    )
}

#[derive(Serialize, FromRow)]
struct TopGifts {
    region: String,
    top_gifts: Vec<String>,
}

async fn top_list(State(pool): State<PgPool>, Path(number): Path<i64>) -> Json<Vec<TopGifts>> {
    Json(
        sqlx::query_as(
            "
            SELECT
                r.name as region,
                ARRAY(
                    SELECT
                        o.gift_name
                    FROM
                        orders o
                    WHERE
                        o.region_id = r.id
                    GROUP BY
                        o.gift_name
                    ORDER BY
                        SUM(o.quantity) DESC
                    LIMIT $1
                ) as top_gifts
            FROM
                regions r
            GROUP BY
                r.id
            ORDER BY
                r.name
            ",
        )
        .bind(number)
        .fetch_all(&pool)
        .await
        .unwrap(),
    )
}

pub fn get_routes() -> Router<PgPool> {
    Router::new()
        .route("/18/reset", routing::post(reset))
        .route("/18/orders", routing::post(crate::day13::orders))
        .route("/18/regions", routing::post(regions))
        .route("/18/regions/total", routing::get(total))
        .route("/18/regions/top_list/:number", routing::get(top_list))
}
