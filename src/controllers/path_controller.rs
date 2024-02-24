use axum::{extract::Path, http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn path(Path(user_id): Path<String>) -> (StatusCode, Json<Value>) {
    let path: Value = json!({ "userId": user_id });

    (StatusCode::OK, Json(path))
}

pub async fn post_path(Path(user_id): Path<String>) -> (StatusCode, Json<Value>) {
    let path: Value = json!({ "userId": user_id });

    (StatusCode::CREATED, Json(path))
}
