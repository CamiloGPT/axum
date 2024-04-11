use axum::{routing::get, Router};

pub fn setup_locations_routes() -> Router {
    Router::new().route("/", get(|| async { "hellow locations" }))
}
