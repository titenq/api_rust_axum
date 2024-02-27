use axum::Json;

use crate::{responses::home_response::HomeResponse, services::home_service::home_message};

pub async fn get_handler() -> Json<HomeResponse> {
    let response: HomeResponse = home_message();

    Json(response)
}
