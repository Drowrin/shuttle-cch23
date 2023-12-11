use axum::routing;
use axum::Json;
use axum::Router;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct ElfResponse {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_no_elf: usize,
}

async fn elf_on_a_shelf(body: String) -> Json<ElfResponse> {
    let elf_on_shelf = body
        .match_indices("on a shelf")
        .filter(|(i, _)| body[(i - 4)..*i].eq("elf "))
        .count();

    let elf = body.matches("elf").count();
    let shelf_no_elf = body.matches("shelf").count() - elf_on_shelf;

    let r = ElfResponse {
        elf,
        elf_on_shelf,
        shelf_no_elf,
    };

    Json(r)
}

pub fn get_routes() -> Router {
    Router::new().route("/6", routing::post(elf_on_a_shelf))
}
