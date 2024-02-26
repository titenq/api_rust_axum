use axum::{response::IntoResponse, Json};

pub async fn home_handler() -> impl IntoResponse {
    const MESSAGE: &str = "RESTful API in Rust using Axum Framework and MongoDB";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
