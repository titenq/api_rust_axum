use chrono::prelude::*;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub published: Option<bool>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct NoteResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub published: bool,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct NoteData {
    pub note: NoteResponse,
}

#[derive(Serialize, Debug)]
pub struct SingleNoteResponse {
    pub status: &'static str,
    pub data: NoteData,
}

#[derive(Serialize, Debug)]
pub struct NoteListResponse {
    pub status: &'static str,
    pub results: usize,
    pub notes: Vec<NoteResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNoteRequest {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateNoteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    #[serde(default = "Utc::now", with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}
