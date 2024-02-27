use mongodb::{bson::Document, options::ClientOptions, Client, Collection, Database};

use crate::{error::MyError, models::note_model::NoteModel};

#[derive(Clone, Debug)]
pub struct DB {
    pub note_collection: Collection<NoteModel>,
    pub note_document: Collection<Document>,
    // pub user_collection: Collection<UserModel>,
}

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn init() -> Result<Self> {
        let mongodb_uri: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let database_name: String =
            std::env::var("MONGO_INITDB_DATABASE").expect("MONGO_INITDB_DATABASE must be set.");

        let mut client_options: ClientOptions = ClientOptions::parse(mongodb_uri).await?;
        client_options.app_name = Some(database_name.to_string());

        let client: Client = Client::with_options(client_options)?;
        let database: Database = client.database(database_name.as_str());
        
        // let user_collection: Collection<UserModel> = database.collection("users");
        let note_collection: Collection<NoteModel> = database.collection::<NoteModel>("notes");
        let note_document: Collection<Document> = database.collection::<Document>("notes");

        println!("✅ Database connected successfully");

        Ok(Self {
            note_collection,
            note_document,
            // user_collection,
        })
    }
}
