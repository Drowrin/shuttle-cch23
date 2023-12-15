use axum::{http::StatusCode, routing, Json, Router};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Params {
    input: String,
}

#[derive(Serialize)]
struct Results {
    result: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

type NaughtyOrNice = Result<Json<Results>, (StatusCode, Json<Results>)>;

async fn nice(Json(Params { input }): Json<Params>) -> NaughtyOrNice {
    let vowels = input.chars().filter(|c| "aeiouy".contains(*c)).count();
    if vowels <= 3 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(Results {
                result: "naughty".to_string(),
                reason: None,
            }),
        ));
    }

    let repeats = input
        .chars()
        .tuple_windows()
        .filter(|(l, r)| l.is_alphabetic() && l == r)
        .count();
    if repeats < 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(Results {
                result: "naughty".to_string(),
                reason: None,
            }),
        ));
    }

    if ["ab", "cd", "pq", "xy"].iter().any(|c| input.contains(c)) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(Results {
                result: "naughty".to_string(),
                reason: None,
            }),
        ));
    }

    return Ok(Json(Results {
        result: "nice".to_string(),
        reason: None,
    }));
}

fn naughty(status: StatusCode, reason: impl ToString) -> NaughtyOrNice {
    Err((
        status,
        Json(Results {
            result: "naughty".to_string(),
            reason: Some(reason.to_string()),
        }),
    ))
}

async fn game(Json(Params { input }): Json<Params>) -> NaughtyOrNice {
    if input.len() < 8 {
        return naughty(StatusCode::BAD_REQUEST, "8 chars");
    }

    let uppercase = input.chars().find(|c| c.is_uppercase()).is_some();
    let lowercase = input.chars().find(|c| c.is_lowercase()).is_some();
    let digits = input.chars().filter(|c| c.is_numeric()).count();
    if !(uppercase && lowercase && digits > 0) {
        return naughty(StatusCode::BAD_REQUEST, "more types of chars");
    }

    if digits < 5 {
        return naughty(StatusCode::BAD_REQUEST, "55555");
    }

    let sum: usize = input
        .split(|c: char| !c.is_numeric())
        .flat_map(|s| s.parse::<usize>())
        .sum();
    if sum != 2023 {
        return naughty(StatusCode::BAD_REQUEST, "math is hard");
    }

    let joy = "joy".to_string();
    if input
        .chars()
        .filter(|c| joy.contains(*c))
        .collect::<String>()
        != joy
    {
        return naughty(StatusCode::NOT_ACCEPTABLE, "not joyful enough");
    }

    if input
        .chars()
        .tuple_windows()
        .find(|(a, b, c)| {
            *a == *c && *a != *b && a.is_alphabetic() && b.is_alphabetic() && c.is_alphabetic()
        })
        .is_none()
    {
        return naughty(
            StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
            "illegal: no sandwich",
        );
    }

    if input
        .chars()
        .find(|c| 10624 <= *c as u32 && *c as u32 <= 11263)
        .is_none()
    {
        return naughty(StatusCode::RANGE_NOT_SATISFIABLE, "outranged");
    }

    if input
        .chars()
        .find(|c| unic::emoji::char::is_emoji(*c) && !c.is_numeric())
        .is_none()
    {
        return naughty(StatusCode::UPGRADE_REQUIRED, "😳");
    }

    if !sha256::digest(input).ends_with("a") {
        return naughty(StatusCode::IM_A_TEAPOT, "not a coffee brewer");
    }

    return Ok(Json(Results {
        result: "nice".to_string(),
        reason: Some("that's a nice password".to_string()),
    }));
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/15/nice", routing::post(nice))
        .route("/15/game", routing::post(game))
}
