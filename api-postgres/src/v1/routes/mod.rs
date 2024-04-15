mod location_routes;

use axum::Router;
use location_routes::setup_locations_routes;

pub fn setup_routes() -> Router {
    Router::new().nest("/locations", setup_locations_routes())
}
