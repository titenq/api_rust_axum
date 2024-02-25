mod routes;
mod structs;
mod controllers;
mod libs;

use axum::Router;
use tokio::net::TcpListener;

use crate::routes::get_router;

#[tokio::main]
async fn main() {
    let app: Router = get_router();

    println!("🚀 Server started successfully");

    let listener: TcpListener = TcpListener::bind("127.0.0.1:5000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
