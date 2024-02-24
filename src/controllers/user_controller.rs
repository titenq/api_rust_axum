use crate::models::user_model::{User, UserBody};
use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn users() -> (StatusCode, Json<Value>) {
    let body: Value = json!([
        { "name": "Leandro" },
        { "name": "João"}
    ]);

    (StatusCode::OK, Json(body))
}

pub async fn post_user(Json(body): Json<UserBody>) -> (StatusCode, Json<User>) {
    let user: User = User {
        id: String::from("ytfuytdftytfyug"),
        name: body.name,
        email: body.email,
        password: body.password,
    };

    (StatusCode::CREATED, Json(user))
}
