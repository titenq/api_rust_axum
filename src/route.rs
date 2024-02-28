use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    handlers::{
        home_handler::get_handler,
        note_handler::{
            create_note_handler, delete_note_handler, edit_note_handler, get_note_handler,
            note_list_handler,
        },
        user_handler::{create_user, delete_user, edit_user, get_user_by_id, get_users},
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let home_route = Router::new()
        .route("/", get(get_handler));

    let route = Router::new()
        .route("/notes", get(note_list_handler)
            .post(create_note_handler))
        .route("/notes/:id", get(get_note_handler)
            .patch(edit_note_handler)
            .delete(delete_note_handler))
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
