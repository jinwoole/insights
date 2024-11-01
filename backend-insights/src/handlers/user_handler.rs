// src/handlers/user_handler.rs


use actix_web::{get, put, web, HttpResponse, Responder};
use mongodb::{Client, bson::doc};
use serde::Deserialize;
use crate::middleware::auth_middleware::AuthorizedUser;
use crate::models::user::User;
use crate::models::user_response::UserResponse;
use crate::utils::hash::hash_password;
use crate::errors::AppError;

// Handler to retrieve user information
#[get("/user")]
pub async fn get_user(
    client: web::Data<Client>, // MongoDB client instance
    user: AuthorizedUser, // Authenticated user data
) -> Result<impl Responder, AppError> {
    // Access the "users" collection in the "mydb" database
    let collection = client.database("mydb").collection::<User>("users");

    // Attempt to find the user by ID
    let user_data = match collection.find_one(doc! { "_id": user.id.clone() }, None).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AppError::UserNotFound),
        Err(e) => {
            println!("Database Find Error: {}", e);
            return Err(AppError::DatabaseError(e.to_string()));
        }
    };

    // Convert the user data to a response format
    let user_response = UserResponse {
        username: user_data.username,
    };

    // Return the user information as a JSON response
    Ok(HttpResponse::Ok().json(user_response))
}

#[derive(Deserialize)]
pub struct UpdateData {
    pub username: Option<String>,
    pub password: Option<String>,
}

// 사용자 정보를 업데이트하는 핸들러
#[put("/user")]
pub async fn update_user(
    client: web::Data<Client>, // MongoDB 클라이언트 인스턴스
    user: AuthorizedUser,      // 인증된 사용자 데이터 (미들웨어에서 검증됨)
    data: web::Json<UpdateData>, // 업데이트할 데이터
) -> Result<impl Responder, AppError> {
    // "mydb" 데이터베이스의 "users" 컬렉션에 접근
    let collection = client.database("mydb").collection::<User>("users");

    // Check if the new username already exists
    if let Some(username) = &data.username {
        // Look for a user with the same username excluding the current user
        let existing_user = collection
            .find_one(
                doc! {
                    "username": username,
                    "_id": { "$ne": &user.id }
                },
                None,
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if existing_user.is_some() {
            return Err(AppError::UserAlreadyExists);
        }
    }

    let mut update_doc = doc! {};

    // 사용자 이름이 제공되면 업데이트 문서에 추가
    if let Some(username) = &data.username {
        update_doc.insert("username", username.clone());
    }

    // 비밀번호가 제공되면 해시하여 업데이트 문서에 추가
    if let Some(password) = &data.password {
        let hashed_password = hash_password(password)?;
        update_doc.insert("password", hashed_password);
    }

    // 사용자 ID로 해당 사용자 업데이트
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
