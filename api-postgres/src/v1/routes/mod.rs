mod health_routes;
mod location_routes;

use axum::Router;
use health_routes::setup_health_routes;
use location_routes::setup_locations_routes;

pub fn setup_routes() -> Router {
    Router::new()
        .nest("/health", setup_health_routes())
        .nest("/locations", setup_locations_routes())
}
