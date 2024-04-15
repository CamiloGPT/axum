use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn get_locations() -> impl IntoResponse {
    const MESSAGE: &str = "Get all locations";

    let json_response = json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
