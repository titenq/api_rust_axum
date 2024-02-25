use axum::{routing::get, Router};

use crate::controllers::user_controller::{create_user, get_user_by_id, get_users};

pub fn get_router() -> Router {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user_by_id))
}
