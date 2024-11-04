// src/main.rs

use actix_web::{App, HttpServer, dev::ServiceRequest, dev::ServiceResponse};
use actix_service::Service; // 추가: Service 트레이트를 가져옵니다.
use actix_web::http::header;
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;
use futures::future::{ok, Ready};

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

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    println!("Server running at http://{}", host);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5174")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap_fn(|req: ServiceRequest, srv| {
                println!("Incoming request: {} {}", req.method(), req.path());
                let fut = srv.call(req);
                async {
                    match fut.await {
                        Ok(res) => {
                            println!(": {}", res.status());
                            Ok(res)
                        },
                        Err(err) => {
                            println!("Request error: {}", err);
                            Err(err)
                        },
                    }
                }
            })
            .app_data(actix_web::web::Data::new(db_client.clone()))
            .configure(handlers::init)
    })
    .bind(host)?
    .run()
    .await
}