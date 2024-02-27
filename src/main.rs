mod routes;
mod controllers;
mod libs;
mod models;

mod db;
mod error;
mod response;
mod route;
mod schema;
mod handlers;
mod services;

use std::sync::Arc;

use tokio::net::TcpListener;
// use axum::{response::IntoResponse, routing::get, Json, Router};
use axum::{http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
}, serve, Router};
use db::DB;
use dotenv::dotenv;
use error::MyError;
use route::create_router;
use tower_http::cors::CorsLayer;

// use crate::routes::get_router;

pub struct AppState {
    db: DB,
}

#[tokio::main]
async fn main() -> Result<(), MyError> {
    dotenv().ok();

    let db: DB = DB::init().await?;
    // let app: Router = get_router();

    let cors: CorsLayer = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app: Router = create_router(Arc::new(AppState { db: db.clone() })).layer(cors);

    println!("🚀 Server started successfully");

    let listener: TcpListener = TcpListener::bind("127.0.0.1:5000").await.unwrap();

    serve(listener, app).await.unwrap();

    Ok(())
}
