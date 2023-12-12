use axum::Router;

mod day00;
mod day01;
mod day04;
mod day06;
mod day07;
mod day08;
mod day11;
mod day12;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(Router::new()
        .merge(day00::get_routes())
        .merge(day01::get_routes())
        .merge(day04::get_routes())
        .merge(day06::get_routes())
        .merge(day07::get_routes())
        .merge(day08::get_routes())
        .merge(day11::get_routes())
        .merge(day12::get_routes())
        .into())
}
