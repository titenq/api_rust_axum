use axum::{routing::get, Router};

use crate::controllers::{
    path_controller::{path, post_path},
    query_controller::query,
    root_controller::root,
    user_controller::{post_user, users},
};

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", get(users).post(post_user))
        .route("/path/:user_id", get(path).post(post_path))
        .route("/query", get(query))
}
