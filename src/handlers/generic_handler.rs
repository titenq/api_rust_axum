use std::sync::Arc;

use axum::{
    extract::{Path, Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use regex::Regex;
use serde_json::{json, Value};

use crate::{
    error::MyError,
    models::{
        note_model::NoteResponse,
        user_model::UserResponse,
    },
    AppState,
};

enum GenericResponse {
    Note(NoteResponse),
    User(UserResponse),
}

impl IntoResponse for GenericResponse {
    fn into_response(self) -> Response {
        match self {
            GenericResponse::Note(note) => Json(note).into_response(),
            GenericResponse::User(user) => Json(user).into_response(),
        }
    }
}

pub async fn get_by_id(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    request: Request,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let url: String = request.uri().to_string();
    let re = Regex::new(r"/([^/]+)/([^/]+)").unwrap();
    let captures = re.captures(&url).unwrap();
    let model = captures.get(1).unwrap().as_str();

    if model == "notes" {
        match app_state
            .db
            .get_note_by_id(&id)
            .await
            .map_err(MyError::from)
        {
            Ok(note) => Ok(GenericResponse::Note(note)),
            Err(e) => Err(e.into()),
        }
    } else if model == "users" {
        match app_state
            .db
            .get_user_by_id(&id)
            .await
            .map_err(MyError::from)
        {
            Ok(user) => Ok(GenericResponse::User(user)),
            Err(e) => Err(e.into()),
        }
    } else {
        Err((StatusCode::NOT_FOUND, Json(json!({ "error": "Not found" }))))
    }
}
