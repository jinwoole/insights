// src/main.rs

use actix_web::{App, HttpServer, dev::ServiceRequest};
use actix_service::Service;
use actix_web::http::header;
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;

mod clients;
mod handlers;
mod middleware;
mod models;
mod utils;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_client = clients::init_db().await.expect("Failed to connect to the database");
    let ses_client = clients::init_ses_client().await.expect("Failed to connect to SES");

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    println!("Server running at http://{}", host);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap_fn(|req: ServiceRequest, srv| {
                let fut = srv.call(req);
                async {
                    match fut.await {
                        Ok(res) => {
                            println!("Response status: {}", res.status());
                            println!("Response headers: {:?}", res.headers());  // 응답 헤더도 확인
                            Ok(res)
                        },
                        Err(err) => {
                            println!("Error: {}", err);
                            Err(err)
                        },
                    }
                }
            })
            .app_data(actix_web::web::Data::new(db_client.clone()))
            .app_data(actix_web::web::Data::new(ses_client.clone()))
            .configure(handlers::init)
    })
    .bind(host)?
    .run()
    .await
}