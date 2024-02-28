use std::sync::Arc;

use crate::{
    error::MyError,
    libs::remove_accent,
    models::user_model::{CreateUserRequest, UpdateUserRequest, UserFilterOptions},
    AppState,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::Value;

pub async fn get_users(
    opts: Option<Query<UserFilterOptions>>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(2) as i64;
    let page = opts.page.unwrap_or(1) as i64;
    let name = opts.name.unwrap_or("".to_string());

    match app_state
        .db
        .get_users_service(limit, page, name)
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn get_user_by_id(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match app_state
        .db
        .get_user_by_id_service(&id)
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn create_user(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match app_state
        .db
        .create_user_service(&body)
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err(e) => Err(e.into()),
    }
}

pub async fn edit_user(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match app_state
        .db
        .edit_user_service(&id, &body)
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn delete_user(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match app_state.db.delete_user_service(&id).await.map_err(MyError::from) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e.into()),
    }
}
