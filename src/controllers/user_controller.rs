use crate::models::user_model::{User, UserBody};
use axum::{extract::Path, http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn get_users() -> (StatusCode, Json<Value>) {
    let body: Value = json!([
        { "name": "Leandro" },
        { "name": "João"}
    ]);

    (StatusCode::OK, Json(body))
}

pub async fn get_user_by_id(Path(id): Path<String>) -> (StatusCode, Json<User>) {
    let user: User = User {
        id,
        name: String::from("Leandro"),
        email: String::from("titenq@gmail.com"),
        password: String::from("123456"),
    };

    (StatusCode::OK, Json(user))
}

pub async fn create_user(Json(body): Json<UserBody>) -> (StatusCode, Json<User>) {
    let user: User = User {
        id: String::from("ytfuytdftytfyug"),
        name: body.name,
        email: body.email,
        password: body.password,
    };

    (StatusCode::CREATED, Json(user))
}
