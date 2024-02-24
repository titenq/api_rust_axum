use std::collections::HashMap;

use axum::{extract::Query, http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn query(Query(params): Query<HashMap<String, String>>) -> (StatusCode, Json<Value>) {
    let params: Value = json!(params);

    (StatusCode::OK, Json(params))
}
