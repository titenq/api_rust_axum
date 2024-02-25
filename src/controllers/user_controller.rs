use std::collections::HashMap;

use crate::{
    libs::remove_accent,
    structs::user_struct::{User, UserBody},
};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};

pub async fn get_users(
    Query(params): Query<HashMap<String, String>>,
) -> (StatusCode, Json<Vec<User>>) {
    let users: Vec<User> = vec![
        User {
            id: String::from("dskjhad38393"),
            name: String::from("Adauto Leandro"),
            email: String::from("titenq@gmail.com"),
            password: String::from("123456"),
        },
        User {
            id: String::from("dskjhad38393"),
            name: String::from("Leandro Ribeiro"),
            email: String::from("leandro@gmail.com"),
            password: String::from("123456"),
        },
        User {
            id: String::from("afseraae3t5435e"),
            name: String::from("João"),
            email: String::from("joao@gmail.com"),
            password: String::from("abcdef"),
        },
    ];

    if params.contains_key("name") {
        let name = params.get("name").unwrap();
        let normalizing_name = remove_accent(&name.to_lowercase());
        let filtered_users: Vec<User> = users
            .iter()
            .filter(|&user| remove_accent(&user.name.to_lowercase()).contains(&normalizing_name))
            .cloned()
            .collect();

        return (StatusCode::OK, Json(filtered_users));
    }

    (StatusCode::OK, Json(users))
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
