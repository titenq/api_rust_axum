mod db;
mod error;
mod handlers;
mod libs;
mod models;
mod route;
mod services;

use std::sync::Arc;

use tokio::net::TcpListener;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    serve, Router,
};
use db::DB;
use dotenv::dotenv;
use error::MyError;
use route::create_router;
use tower_http::cors::CorsLayer;

pub struct AppState {
    db: DB,
}

#[tokio::main]
async fn main() -> Result<(), MyError> {
    dotenv().ok();

    let db: DB = DB::init().await?;

    let cors: CorsLayer = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app: Router = create_router(Arc::new(AppState { db: db.clone() })).layer(cors);

    let listener: TcpListener = TcpListener::bind("127.0.0.1:5000").await.unwrap();

    println!("🚀 Server started successfully");
    
    serve(listener, app).await.unwrap();

    Ok(())
}
