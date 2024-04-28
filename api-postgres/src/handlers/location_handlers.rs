use crate::{services::location_services, AppState};
use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;
use std::sync::Arc;

pub async fn get_locations(State(data): State<Arc<AppState>>) -> impl IntoResponse {
    const MESSAGE: &str = "Get all locations";

    let result = location_services::get_locations();

    let json_response = json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
