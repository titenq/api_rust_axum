use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    handlers::{
        home_handler::get_handler,
        note_handler::{
            create_note_handler, delete_note_handler, edit_note_handler, get_note_handler,
            note_list_handler,
        },
    },
    AppState,
};

/* pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_handler))
        .route("/notes", get(note_list_handler)
            .post(create_note_handler))
        .route("/notes/:id", get(get_note_handler)
            .patch(edit_note_handler)
            .delete(delete_note_handler),
        )
        .with_state(app_state)
} */

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let home_route = Router::new().route("/", get(get_handler));

    let routes = Router::new()
        .route("/notes", get(note_list_handler)
            .post(create_note_handler))
        .route("/notes/:id", get(get_note_handler)
            .patch(edit_note_handler)
            .delete(delete_note_handler),
        )
        .with_state(app_state);

        Router::new().nest("/", home_route).nest("/", routes)
}

