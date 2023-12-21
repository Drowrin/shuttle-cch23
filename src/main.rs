use axum::Router;
use sqlx::PgPool;

mod day00;
mod day01;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day18;
mod day19;
mod day20;
mod day21;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(local_uri = "{secrets.LOCAL_DB_URI}")] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    Ok(Router::new()
        .merge(day00::get_routes())
        .merge(day01::get_routes())
        .merge(day05::get_routes())
        .merge(day04::get_routes())
        .merge(day06::get_routes())
        .merge(day07::get_routes())
        .merge(day08::get_routes())
        .merge(day11::get_routes())
        .merge(day12::get_routes())
        .merge(day13::get_routes())
        .merge(day14::get_routes())
        .merge(day15::get_routes())
        .merge(day18::get_routes())
        .merge(day19::get_routes())
        .merge(day20::get_routes())
        .merge(day21::get_routes())
        .with_state(pool)
        .into())
}
