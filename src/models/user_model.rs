use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct UserBody {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}
