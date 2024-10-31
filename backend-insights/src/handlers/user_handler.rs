// src/handlers/user_handler.rs

use actix_web::{get, put, web, HttpResponse, Responder};
use mongodb::{Client, bson::doc};
use serde::Deserialize;
use crate::middleware::auth_middleware::AuthorizedUser;
use crate::models::user::User;
use crate::models::user_response::UserResponse;
use crate::utils::hash::hash_password;
use crate::errors::AppError;

#[get("/user")]
pub async fn get_user(
    client: web::Data<Client>,
    user: AuthorizedUser,
) -> Result<impl Responder, AppError> {
    let collection = client.database("mydb").collection::<User>("users");

    let user_data = match collection.find_one(doc! { "_id": user.id.clone() }, None).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AppError::UserNotFound),
        Err(e) => {
            println!("Database Find Error: {}", e);
            return Err(AppError::DatabaseError(e.to_string()));
        }
    };

    // UserResponse로 변환하여 응답
    let user_response = UserResponse {
        id: user_data.id,
        username: user_data.username,
    };

    Ok(HttpResponse::Ok().json(user_response))
}

#[derive(Deserialize)]
pub struct UpdateData {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[put("/user")]
pub async fn update_user(
    client: web::Data<Client>,
    user: AuthorizedUser,
    data: web::Json<UpdateData>,
) -> Result<impl Responder, AppError> {
    let collection = client.database("mydb").collection::<User>("users");

    let mut update_doc = doc! {};

    if let Some(username) = &data.username {
        update_doc.insert("username", username.clone());
    }

    if let Some(password) = &data.password {
        let hashed_password = hash_password(password)?;
        update_doc.insert("password", hashed_password);
    }

    match collection.update_one(
        doc! { "_id": user.id.clone() },
        doc! { "$set": update_doc },
        None,
    ).await {
        Ok(_) => Ok(HttpResponse::Ok().json("User updated successfully")),
        Err(e) => {
            println!("Database Update Error: {}", e);
            Err(AppError::DatabaseError(e.to_string()))
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(crate::middleware::auth_middleware::AuthMiddleware)
            .service(get_user)
            .service(update_user),
    );
}
