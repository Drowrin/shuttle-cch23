use axum::Router;
use sqlx::PgPool;

mod day00;
mod day01;
mod day04;
mod day06;
mod day07;
mod day08;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(local_uri = "{secrets.LOCAL_DB_URI}")] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    Ok(Router::new()
        .merge(day00::get_routes())
        .merge(day01::get_routes())
        .merge(day04::get_routes())
        .merge(day06::get_routes())
        .merge(day07::get_routes())
        .merge(day08::get_routes())
        .merge(day11::get_routes())
        .merge(day12::get_routes())
        .merge(day13::get_routes(pool))
        .merge(day14::get_routes())
        .merge(day15::get_routes())
        .into())
}
