use crate::handlers::health_handlers;
use axum::{routing::get, Router};

pub fn setup_health_routes() -> Router {
    Router::new().route("/", get(health_handlers::get_health))
}
