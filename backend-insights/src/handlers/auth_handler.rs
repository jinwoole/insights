// src/handlers/auth_handler.rs

use actix_web::{post, web, HttpResponse, Responder};
use mongodb::Client;
use serde::Deserialize;
use bson::doc;
use crate::models::user::User;
use crate::models::auth::AuthResponse;
use crate::utils::{hash::hash_password, jwt::generate_jwt};
use crate::errors::AppError;
use bson::oid::ObjectId;

#[derive(Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

#[post("/auth/register")]
pub async fn register(
    client: web::Data<Client>,
    data: web::Json<AuthData>,
) -> Result<impl Responder, AppError> {
    let collection = client.database("mydb").collection::<User>("users");

    let hashed_password = hash_password(&data.password)?;

    let new_user = User {
        id: ObjectId::new(),
        username: data.username.clone(),
        password: hashed_password,
    };

    match collection.insert_one(&new_user, None).await {
        Ok(_) => Ok(HttpResponse::Ok().json("User registered successfully")),
        Err(e) => {
            println!("Database Insert Error: {}", e);
            Err(AppError::DatabaseError(e.to_string()))
        }
    }
}

#[post("/auth/login")]
pub async fn login(
    client: web::Data<Client>,
    data: web::Json<AuthData>,
) -> Result<impl Responder, AppError> {
    let collection = client.database("mydb").collection::<User>("users");

    let user = match collection.find_one(doc! { "username": &data.username }, None).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AppError::UserNotFound),
        Err(e) => {
            println!("Database Find Error: {}", e);
            return Err(AppError::DatabaseError(e.to_string()));
        }
    };

    if crate::utils::hash::verify_password(&data.password, &user.password)? {
        let token = generate_jwt(user.id.to_hex().as_str())?;
        let response = AuthResponse { token };
        Ok(HttpResponse::Ok().json(response))
    } else {
        Err(AppError::InvalidCredentials)
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
}
