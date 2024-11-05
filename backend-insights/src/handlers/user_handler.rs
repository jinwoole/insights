use actix_web::{get, web, HttpResponse, Responder};
use mongodb::{Client, bson::doc};
use crate::models::user::User;
use crate::middleware::auth_middleware::AuthorizedUser;
use actix_web::post;
use serde::Deserialize;
use crate::errors::AppError;


#[get("/get")]
pub async fn get_user(
    client: web::Data<Client>,
    user: AuthorizedUser,
) -> Result<impl Responder, AppError> {
    let users = client.database("insights").collection::<User>("users");
    let user_data = users.find_one(doc! { "_id": &user.id }, None).await?
        .ok_or(AppError::UserNotFound)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "username": user_data.username,
        "email": user_data.email
    })))
}

#[derive(Deserialize)]
pub struct UsernameUpdate {
    pub username: String,
}

#[post("/username")]
pub async fn change_username(
    client: web::Data<Client>,
    user: AuthorizedUser,
    new_data: web::Json<UsernameUpdate>,
) -> Result<impl Responder, AppError> {
    let users = client.database("insights").collection::<User>("users");
    let new_username = &new_data.username;

    let current_user = users.find_one(doc! { "_id": &user.id }, None).await?
        .ok_or(AppError::UserNotFound)?;

    if new_username.trim().is_empty() {
        return Err(AppError::InvalidToken);
    }

    if new_username == &current_user.username {
        return Err(AppError::UserAlreadyExists);
    }

    if users.find_one(doc! { "username": new_username }, None).await?.is_some() {
        return Err(AppError::UserAlreadyExists);
    }

    users.update_one(
        doc! { "_id": &user.id },
        doc! { "$set": { "username": new_username } },
        None,
    ).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "response": "ok" })))
}

