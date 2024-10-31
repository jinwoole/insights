// src/main.rs

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod db;
mod handlers;
mod middleware;
mod models;
mod utils;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_client = db::init_db().await.expect("Failed to connect to the database");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db_client.clone()))
            .configure(handlers::init)
    });

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    println!("Server running at http://{}", host);

    server.bind(host)?.run().await
}
