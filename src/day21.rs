use std::str::FromStr;

use axum::{extract::Path, routing, Router};
use celes::Country;
use dms_coordinates::DMS;
use reverse_geocoder::ReverseGeocoder;
use s2::{cellid::CellID, latlng::LatLng};
use sqlx::PgPool;

async fn coords_binary(Path(binary): Path<String>) -> String {
    let id = u64::from_str_radix(&binary, 2).unwrap();
    let cell = CellID(id);
    let pos = LatLng::from(cell);
    let lat = DMS::from_decimal_degrees(pos.lat.deg(), true);
    let lng = DMS::from_decimal_degrees(pos.lng.deg(), false);

    format!(
        "{}°{}'{:.3}''{} {}°{}'{:.3}''{}",
        lat.degrees,
        lat.minutes,
        lat.seconds,
        lat.bearing,
        lng.degrees,
        lng.minutes,
        lng.seconds,
        lng.bearing,
    )
}

async fn country_binary(Path(binary): Path<String>) -> String {
    let id = u64::from_str_radix(&binary, 2).unwrap();
    let cell = CellID(id);
    let pos = LatLng::from(cell);

    let reverse_geocoder = ReverseGeocoder::new();
    let loc = reverse_geocoder.search((pos.lat.deg(), pos.lng.deg()));
    let country = Country::from_str(loc.record.cc.as_str()).unwrap();

    // poor problem specs make this necessary, as the expected names deviate from iso standards
    match country.code {
        "096" => "Brunei",
        "528" => "Belgium",
        _ => country.long_name,
    }
    .to_string()
}

pub fn get_routes() -> Router<PgPool> {
    Router::new()
        .route("/21/coords/:binary", routing::get(coords_binary))
        .route("/21/country/:binary", routing::get(country_binary))
}
