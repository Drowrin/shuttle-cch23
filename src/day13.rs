use std::collections::HashMap;

use axum::{extract::State, routing, Json, Router};
use serde::Deserialize;
use sqlx::{PgPool, Row};

async fn sql(State(pool): State<PgPool>) -> String {
    sqlx::query("SELECT 20231213")
        .fetch_one(&pool)
        .await
        .unwrap()
        .get::<i32, _>(0)
        .to_string()
}

async fn reset(State(pool): State<PgPool>) {
    sqlx::query("DROP TABLE IF EXISTS orders;")
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
pub struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

pub async fn orders(State(pool): State<PgPool>, Json(orders): Json<Vec<Order>>) {
    sqlx::QueryBuilder::new("INSERT INTO orders (id, region_id, gift_name, quantity) ")
        .push_values(orders, |mut builder, order| {
            builder
                .push_bind(order.id)
                .push_bind(order.region_id)
                .push_bind(order.gift_name)
                .push_bind(order.quantity);
        })
        .build()
        .execute(&pool)
        .await
        .unwrap();
}

async fn total(State(pool): State<PgPool>) -> Json<HashMap<String, i64>> {
    Json(HashMap::from([(
        "total".to_string(),
        sqlx::query(
            "
            SELECT SUM (quantity)
            FROM orders;
            ",
        )
        .fetch_one(&pool)
        .await
        .unwrap()
        .get(0),
    )]))
}

async fn popular(State(pool): State<PgPool>) -> Json<HashMap<String, Option<String>>> {
    Json(HashMap::from([(
        "popular".to_string(),
        match sqlx::query(
            "
        SELECT gift_name
        FROM orders
        GROUP BY gift_name
        ORDER BY SUM(quantity) DESC
        LIMIT 1;
        ",
        )
        .fetch_one(&pool)
        .await
        {
            Ok(row) => Some(row.get(0)),
            _ => None,
        },
    )]))
}

pub fn get_routes() -> Router<PgPool> {
    Router::new()
        .route("/13/sql", routing::get(sql))
        .route("/13/reset", routing::post(reset))
        .route("/13/orders", routing::post(orders))
        .route("/13/orders/total", routing::get(total))
        .route("/13/orders/popular", routing::get(popular))
}
