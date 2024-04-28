use crate::handlers::location_handlers;
use crate::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn setup_locations_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(location_handlers::get_locations))
        .with_state(app_state)
}
