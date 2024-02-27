use std::str::FromStr;

use chrono::prelude::*;
use futures::StreamExt;
use mongodb::{bson::{self, doc, oid::ObjectId}, options::{FindOneAndUpdateOptions, FindOptions, IndexOptions, ReturnDocument}, IndexModel};

use crate::{db::DB, error::MyError::{self, *}, model::NoteModel, response::{NoteData, NoteListResponse, NoteResponse, SingleNoteResponse}, schema::{CreateNoteSchema, UpdateNoteSchema}};

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn get_all_notes(&self, limit: i64, page: i64) -> Result<NoteListResponse> {
        let find_options = FindOptions::builder()
            .limit(limit)
            .skip(u64::try_from((page - 1) * limit).unwrap())
            .build();

        let mut cursor = self
            .note_collection
            .find(None, find_options)
            .await
            .map_err(MongoQueryError)?;

        let mut json_result: Vec<NoteResponse> = Vec::new();

        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_note_service(&doc.unwrap())?);
        }

        Ok(NoteListResponse {
            status: "success",
            results: json_result.len(),
            notes: json_result,
        })
    }

    pub async fn get_note_by_id(&self, id: &str) -> Result<NoteResponse> {
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        let note_doc = self
            .note_collection
            .find_one(doc! {"_id":oid }, None)
            .await
            .map_err(MongoQueryError)?;

        match note_doc {
            Some(doc) => {
                let note = self.doc_to_note_service(&doc)?;
                Ok(note)
            }
            None => Err(NotFoundError(id.to_string())),
        }
    }

    pub async fn create_note(&self, body: &CreateNoteSchema) -> Result<SingleNoteResponse> {
        let published = body.published.to_owned().unwrap_or(false);
        let category = body.category.to_owned().unwrap_or_default();

        let document = self.create_note_document_service(body, published, category)?;

        let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder()
            .keys(doc! {"title": 1})
            .options(options)
            .build();

        match self.note_collection.create_index(index, None).await {
            Ok(_) => {}
            Err(e) => return Err(MongoQueryError(e)),
        };

        let insert_result = match self.collection.insert_one(&document, None).await {
            Ok(result) => result,
            Err(e) => {
                if e.to_string()
                    .contains("E11000 duplicate key error collection")
                {
                    return Err(MongoDuplicateError(e));
                }
                return Err(MongoQueryError(e));
            }
        };

        let new_id = insert_result
            .inserted_id
            .as_object_id()
            .expect("issue with new _id");

        let note_doc = match self
            .note_collection
            .find_one(doc! {"_id": new_id}, None)
            .await
        {
            Ok(Some(doc)) => doc,
            Ok(None) => return Err(NotFoundError(new_id.to_string())),
            Err(e) => return Err(MongoQueryError(e)),
        };

        Ok(SingleNoteResponse {
            status: "success",
            data: NoteData {
                note: self.doc_to_note_service(&note_doc)?,
            },  
        })
    }

    pub async fn edit_note(&self, id: &str, body: &UpdateNoteSchema) -> Result<SingleNoteResponse> {
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        let mut update = doc! {
            "$set": bson::to_document(body).map_err(MongoSerializeBsonError)?,
        };

        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        if let Some(doc) = self
            .note_collection
            .find_one_and_update(doc! {"_id": oid}, update, options)
            .await
            .map_err(MongoQueryError)?
        {
            let note = self.doc_to_note_service(&doc)?;
            let note_response = SingleNoteResponse {
                status: "success",
                data: NoteData { note },
            };
            Ok(note_response)
        } else {
            Err(NotFoundError(id.to_string()))
        }
    }

    fn doc_to_note_service(&self, note: &NoteModel) -> Result<NoteResponse> {
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

    fn create_note_document_service(
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
