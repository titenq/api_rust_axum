use crate::models::home_model::HomeResponse;

pub fn home_message() -> HomeResponse {
    const MESSAGE: &str = "RESTful API in Rust using Axum Framework and MongoDB";

    let response: HomeResponse = HomeResponse {
        status: "success".to_owned(),
        message: MESSAGE.to_owned(),
    };

    response
}
