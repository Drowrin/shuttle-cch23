use axum::Router;

mod day00;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(Router::new().merge(day00::get_routes()).into())
}
