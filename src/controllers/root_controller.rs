use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn root() -> (StatusCode, Json<Value>) {
    let body: Value = json!({ "message": "API online" });

    (StatusCode::OK, Json(body))
}
