use serde::Serialize;

#[derive(Serialize)]
pub struct HomeResponse {
    pub status: String,
    pub message: String,
}
