use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    handlers::{
        generic_handler::get_by_id, home_handler::get_handler, note_handler::{
            create_note, delete_note, edit_note, note_list
        }, user_handler::{create_user, delete_user, edit_user, get_users}
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let home_route = Router::new()
        .route("/", get(get_handler));

    let route = Router::new()
        .route("/notes", get(note_list)
            .post(create_note))
        .route("/notes/:id", get(get_by_id)
            .patch(edit_note)
            .delete(delete_note),
        )
        .route("/users", get(get_users)
            .post(create_user))
        .route("/users/:id", get(get_by_id)
            .patch(edit_user)
            .delete(delete_user),
        )
        .with_state(app_state);

    Router::new().nest("/", home_route).nest("/", route)
}
