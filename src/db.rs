use crate::{error::MyError, models::note_model::NoteModel};
use mongodb::{bson::Document, options::ClientOptions, Client, Collection, Database};

#[derive(Clone, Debug)]
pub struct DB {
    pub note_collection: Collection<NoteModel>,
    pub collection: Collection<Document>,
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

        let note_collection: Collection<NoteModel> = database.collection("notes");
        let collection: Collection<Document> = database.collection::<Document>("notes");

        println!("✅ Database connected successfully");

        Ok(Self {
            note_collection,
            collection,
        })
    }
}
