use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn get_health() -> impl IntoResponse {
    const MESSAGE: &str = "API is OK";

    let json_response = json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
