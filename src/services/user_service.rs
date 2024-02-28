use std::str::FromStr;

use chrono::prelude::*;
use futures::StreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson, Document},
    options::{FindOneAndUpdateOptions, FindOptions, IndexOptions, ReturnDocument},
    results::{DeleteResult, InsertOneResult},
    Cursor, IndexModel,
};

use crate::{
    db::DB, error::MyError::{self, *}, libs::remove_accent, models::user_model::{CreateUserRequest, UpdateUserRequest, UserListResponse, UserModel, UserResponse}
};

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn get_users_service(&self, limit: i64, page: i64, name: String) -> Result<UserListResponse> {
        let find_options: FindOptions = FindOptions::builder()
            .limit(limit)
            .skip(u64::try_from((page - 1) * limit).unwrap())
            .build();
        let mut query_filter = doc! {};

        if !name.is_empty() {
            let regex = format!(".*{}.*", remove_accent(&name));
            
            query_filter.insert("name", doc! { "$regex": regex, "$options": "i" });
        }

        let count = self.user_collection.count_documents(query_filter.clone(), None).await?;
    
        let mut cursor: Cursor<UserModel> = self
            .user_collection
            .find(Some(query_filter), find_options)
            .await
            .map_err(MongoQueryError)?;
    
        let mut json_result: Vec<UserResponse> = Vec::new();
    
        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_user_service(&doc.unwrap())?);
        }
    
        Ok(UserListResponse {
            currentPage: page as usize,
            totalPages: count.div_ceil(limit as u64) as usize,
            users: json_result,
        })
    }

    pub async fn get_user_by_id_service(&self, id: &str) -> Result<UserResponse> {
        let oid: ObjectId = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        let user_doc: Option<UserModel> = self
            .user_collection
            .find_one(doc! { "_id": oid }, None)
            .await
            .map_err(MongoQueryError)?;

        match user_doc {
            Some(doc) => {
                let user: UserResponse = self.doc_to_user_service(&doc)?;
                Ok(user)
            }
            None => Err(NotFoundError(id.to_string())),
        }
    }

    pub async fn create_user_service(&self, body: &CreateUserRequest) -> Result<UserResponse> {
        let is_active: bool = body.isActive.to_owned().unwrap_or(false);

        let document: Document = self.create_user_document_service(body, is_active)?;

        let options: IndexOptions = IndexOptions::builder().unique(true).build();
        let index: IndexModel = IndexModel::builder()
            .keys(doc! {"email": 1})
            .options(options)
            .build();

        match self.user_collection.create_index(index, None).await {
            Ok(_) => {}
            Err(e) => return Err(MongoQueryError(e)),
        };

        let insert_result: InsertOneResult =
            match self.user_document.insert_one(&document, None).await {
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

        let new_id: ObjectId = insert_result
            .inserted_id
            .as_object_id()
            .expect("issue with new _id");

        let user_doc: UserModel = match self
            .user_collection
            .find_one(doc! {"_id": new_id}, None)
            .await
        {
            Ok(Some(doc)) => doc,
            Ok(None) => return Err(NotFoundError(new_id.to_string())),
            Err(e) => return Err(MongoQueryError(e)),
        };

        let user: UserResponse = self.doc_to_user_service(&user_doc)?;

        Ok(user)
    }

    pub async fn edit_user_service(
        &self,
        id: &str,
        body: &UpdateUserRequest,
    ) -> Result<UserResponse> {
        let oid: ObjectId = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        let update: Document = doc! {
            "$set": bson::to_document(body).map_err(MongoSerializeBsonError)?,
        };

        let options: FindOneAndUpdateOptions = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        if let Some(doc) = self
            .user_collection
            .find_one_and_update(doc! {"_id": oid}, update, options)
            .await
            .map_err(MongoQueryError)?
        {
            let user_response: UserResponse = self.doc_to_user_service(&doc)?;

            Ok(user_response)
        } else {
            Err(NotFoundError(id.to_string()))
        }
    }

    pub async fn delete_user_service(&self, id: &str) -> Result<()> {
        let oid: ObjectId = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let filter: Document = doc! {"_id": oid };

        let result: DeleteResult = self
            .user_document
            .delete_one(filter, None)
            .await
            .map_err(MongoQueryError)?;

        match result.deleted_count {
            0 => Err(NotFoundError(id.to_string())),
            _ => Ok(()),
        }
    }

    fn doc_to_user_service(&self, user: &UserModel) -> Result<UserResponse> {
        let user_response: UserResponse = UserResponse {
            id: user.id.to_hex(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            password: user.password.to_owned(),
            roles: user.roles.clone(),
            isActive: user.isActive,
            createdAt: user.createdAt,
            updatedAt: user.updatedAt,
        };

        Ok(user_response)
    }

    fn create_user_document_service(
        &self,
        body: &CreateUserRequest,
        is_active: bool
    ) -> Result<bson::Document> {
        let serialized_data: Bson = bson::to_bson(body).map_err(MongoSerializeBsonError)?;
        let document: &Document = serialized_data.as_document().unwrap();

        let datetime: DateTime<Utc> = Utc::now();

        let mut doc_with_dates: Document = doc! {
            "createdAt": datetime,
            "updatedAt": datetime,
            "isActive": is_active
        };
        doc_with_dates.extend(document.clone());

        Ok(doc_with_dates)
    }
}
