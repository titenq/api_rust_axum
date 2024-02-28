use chrono::prelude::*;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<String>,
    pub isActive: Option<bool>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<String>,
    pub isActive: Option<bool>,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct UserListResponse {
    pub currentPage: usize,
    pub totalPages: usize,
    pub users: Vec<UserResponse>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isActive: Option<bool>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isActive: Option<bool>,
    #[serde(default = "Utc::now", with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Default)]
pub struct UserFilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>
}
