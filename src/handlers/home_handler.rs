use axum::Json;

use crate::{models::home_model::HomeResponse, services::home_service::home_message};

pub async fn home() -> Json<HomeResponse> {
    let response: HomeResponse = home_message();

    Json(response)
}
