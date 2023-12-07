use axum::routing;
use axum::Json;
use axum::Router;
use serde::Serialize;

#[derive(Serialize)]
struct ElfResponse {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_no_elf: usize,
}

async fn elf_on_a_shelf(body: String) -> Json<ElfResponse> {
    let elf_on_shelf = body.matches("elf on a shelf").count();
    Json(ElfResponse {
        elf: body.matches("elf").count(),
        elf_on_shelf,
        shelf_no_elf: body.matches("shelf").count() - elf_on_shelf,
    })
}

pub fn get_routes() -> Router {
    Router::new().route("/6", routing::post(elf_on_a_shelf))
}
