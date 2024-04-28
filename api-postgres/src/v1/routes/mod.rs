mod health_routes;
mod location_routes;

use crate::AppState;
use axum::Router;
use health_routes::setup_health_routes;
use location_routes::setup_locations_routes;
use std::sync::Arc;

pub fn setup_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/health", setup_health_routes())
        .nest("/locations", setup_locations_routes(app_state))
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new().nest("/api/v1", setup_routes(app_state))
}
