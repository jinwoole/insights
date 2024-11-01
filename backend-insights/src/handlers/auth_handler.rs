

use actix_web::{post, web, HttpResponse, Responder, cookie::Cookie};
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

    // Check for existing username
    if collection
        .find_one(doc! { "username": &data.username }, None)
        .await?
        .is_some()
    {
        return Err(AppError::UserAlreadyExists);
    }

    let hashed_password = hash_password(&data.password)?;

    let new_user = User {
        id: ObjectId::new(),
        username: data.username.clone(),
        password: hashed_password,
    };

    match collection.insert_one(&new_user, None).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "ok" }))),
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
        let cookie = Cookie::build("token", token)
            .path("/")
            .http_only(true)
            .secure(true) // HTTPS 사용 시 true로 설정
            .same_site(actix_web::cookie::SameSite::Strict)
            .finish();

        Ok(HttpResponse::Ok()
            .cookie(cookie)
            .finish())
    } else {
        Err(AppError::InvalidCredentials)
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
}
