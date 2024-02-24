use axum::{routing::get, Router};

use crate::controllers::{
    query_controller::query,
    root_controller::root,
    user_controller::{create_user, get_user_by_id, get_users},
};

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user_by_id))
        .route("/query", get(query))
}
