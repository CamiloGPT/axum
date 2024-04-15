use crate::handlers::location_handlers;
use axum::{routing::get, Router};

pub fn setup_locations_routes() -> Router {
    Router::new().route("/", get(location_handlers::get_locations))
}
