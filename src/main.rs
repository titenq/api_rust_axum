#[macro_use]
mod macros;

mod controllers;
mod models;
mod routes;

use axum::Router;
use tokio::net::TcpListener;

use crate::routes::get_router;

#[tokio::main]
async fn main() {
    let app: Router = get_router();

    let test_macro = "Testando macro print!";
    let test_macro_2 = string!("Testando macro string!");
    print!(test_macro);
    print!(test_macro_2);

    println!("🚀 Server started successfully");

    let listener: TcpListener = TcpListener::bind("127.0.0.1:5000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
