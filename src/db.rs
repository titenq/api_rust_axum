use crate::error::MyError;
use crate::response::{NoteData, NoteListResponse, NoteResponse, SingleNoteResponse};
use crate::{
    error::MyError::*, model::NoteModel, schema::CreateNoteSchema, schema::UpdateNoteSchema,
};
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, IndexOptions, ReturnDocument};
use mongodb::{bson, options::ClientOptions, Client, Collection, IndexModel};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct DB {
    pub note_collection: Collection<NoteModel>,
    pub collection: Collection<Document>,
}

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn init() -> Result<Self> {
        let mongodb_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let database_name =
            std::env::var("MONGO_INITDB_DATABASE").expect("MONGO_INITDB_DATABASE must be set.");

        let mut client_options = ClientOptions::parse(mongodb_uri).await?;
        client_options.app_name = Some(database_name.to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database(database_name.as_str());

        let note_collection: Collection<NoteModel> = database.collection("notes");
        let collection: Collection<Document> = database.collection::<Document>("notes");

        println!("✅ Database connected successfully");

        Ok(Self {
            note_collection,
            collection,
        })
    }

    pub async fn delete_note(&self, id: &str) -> Result<()> {
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let filter = doc! {"_id": oid };

        let result = self
            .collection
            .delete_one(filter, None)
            .await
            .map_err(MongoQueryError)?;

        match result.deleted_count {
            0 => Err(NotFoundError(id.to_string())),
            _ => Ok(()),
        }
    }

    fn doc_to_note(&self, note: &NoteModel) -> Result<NoteResponse> {
        let note_response = NoteResponse {
            id: note.id.to_hex(),
            title: note.title.to_owned(),
            content: note.content.to_owned(),
            category: note.category.to_owned().unwrap(),
            published: note.published.unwrap(),
            createdAt: note.createdAt,
            updatedAt: note.updatedAt,
        };

        Ok(note_response)
    }

    fn create_note_document(
        &self,
        body: &CreateNoteSchema,
        published: bool,
        category: String,
    ) -> Result<bson::Document> {
        let serialized_data = bson::to_bson(body).map_err(MongoSerializeBsonError)?;
        let document = serialized_data.as_document().unwrap();

        let datetime = Utc::now();

        let mut doc_with_dates = doc! {
            "createdAt": datetime,
            "updatedAt": datetime,
            "published": published,
            "category": category
        };
        doc_with_dates.extend(document.clone());

        Ok(doc_with_dates)
    }
}
