use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    handlers::{
        home_handler::home,
        note_handler::{
            create_note, delete_note, edit_note, get_note_by_id, get_notes,
        },
        user_handler::{create_user, delete_user, edit_user, get_user_by_id, get_users},
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let home_route = Router::new()
        .route("/", get(home));

    let route = Router::new()
        .route("/notes", get(get_notes)
            .post(create_note))
        .route("/notes/:id", get(get_note_by_id)
            .patch(edit_note)
            .delete(delete_note))
        .route("/users", get(get_users)
            .post(create_user))
        .route("/users/:id", get(get_user_by_id)
            .patch(edit_user)
            .delete(delete_user))
        .with_state(app_state);

    Router::new()
        .nest("/", home_route)
        .nest("/", route)
}
