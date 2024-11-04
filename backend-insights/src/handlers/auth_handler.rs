use core::prelude;

use actix_web::{post, web, HttpResponse, Responder, cookie::Cookie};
use mongodb::Client;
use serde::{Deserialize, Serialize};
use mongodb::bson::{doc, Document};
use mongodb::options::{IndexOptions};
use mongodb::IndexModel;
use chrono::{Utc, DateTime};
use rand::Rng;



use crate::models::user::User;
use crate::models::auth::AuthResponse;
use crate::utils::{hash::hash_password, jwt::generate_jwt};
use crate::errors::AppError;
use bson::oid::ObjectId;


// Separate structs for registration and login
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
    pub email: String,
}

// Modify VerificationData to include action
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationRegisterData {
    pub username: String,
    pub email: String,
    pub code: u32,
}

// Modify VerificationData to include action
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationLoginData {
    pub email: String,
    pub code: u32,
}


// First endpoint: receive username and email, generate code, store in cache
#[post("/auth/register")]
pub async fn register(
    client: web::Data<Client>,
    data: web::Json<RegisterData>,
) -> Result<impl Responder, AppError> {
    let users = client.database("mydb").collection::<User>("users");
    let cache = client.database("mydb").collection::<Document>("cache");

    // Create TTL index on "createdAt" field
    let index_options = IndexOptions::builder()
        .expire_after(Some(std::time::Duration::from_secs(180)))
        .build();
    let index_model = IndexModel::builder()
        .keys(doc! { "createdAt": 1 })
        .options(index_options)
        .build();
    cache.create_index(index_model, None).await?;

    // Check for existing username
    if users
        .find_one(doc! { "username": &data.username }, None)
        .await?
        .is_some()
    {
        return Err(AppError::UserAlreadyExists);
    }

    // Check for existing email
    if users
        .find_one(doc! { "email": &data.email }, None)
        .await?
        .is_some()
    {
        return Err(AppError::EmailAlreadyExists);
    }

    let existing = cache.find_one(doc! { "email": &data.email }, None).await?;
    if existing.is_some() {
        cache.delete_one(doc! { "email": &data.email }, None).await?;
    }

    // Generate random 6-digit code
    let code: u32 = rand::thread_rng().gen_range(100_000..1_000_000);

    let cache_entry = doc! {
        "username": &data.username,
        "email": &data.email,
        "code": code,
        "createdAt": bson::DateTime::from_millis(Utc::now().timestamp_millis()),
    };

    cache.insert_one(cache_entry, None).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "ok" })))
}

// Verify Registration
#[post("/auth/verify/register")]
pub async fn verify_register(
    client: web::Data<Client>,
    data: web::Json<VerificationRegisterData>,
) -> Result<impl Responder, AppError> {
    let cache = client.database("mydb").collection::<Document>("cache");

    // Check if code is valid
    let filter = doc! {
        "username": &data.username,
        "email": &data.email,
        "code": data.code,
    };

    let cache_entry = cache.find_one(filter.clone(), None).await?;
    if let Some(entry) = cache_entry {
        let username = entry.get_str("username").map_err(|_| AppError::InvalidOrExpiredCode)?.to_string();

        let users = client.database("mydb").collection::<User>("users");

        // Create new user
        let new_user = User {
            id: ObjectId::new(),
            username,
            email: data.email.clone(),
            // Initialize other fields as needed
        };

        users.insert_one(&new_user, None).await?;

        // Remove cache entry
        cache.delete_one(filter, None).await?;

        let token = generate_jwt(&new_user.id.to_hex())?;
        let cookie = Cookie::build("token", token)
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(actix_web::cookie::SameSite::Strict)
            .finish();

        Ok(HttpResponse::Ok()
            .cookie(cookie)
            .finish())
    } else {
        Err(AppError::InvalidOrExpiredCode)
    }
}

// Verify Login
#[post("/auth/verify/login")]
pub async fn verify_login(
    client: web::Data<Client>,
    data: web::Json<VerificationLoginData>,
) -> Result<impl Responder, AppError> {
    let cache = client.database("mydb").collection::<Document>("cache");

    // Check if code is valid
    let filter = doc! {
        "email": &data.email,
        "code": data.code,
    };

    let cache_entry = cache.find_one(filter.clone(), None).await?;
    if let Some(_entry) = cache_entry {
        let users = client.database("mydb").collection::<User>("users");
        let user = users.find_one(doc! { "email": &data.email }, None).await?
            .ok_or(AppError::UserNotFound)?;

        let token = generate_jwt(&user.id.to_hex())?;
        let cookie = Cookie::build("token", token)
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(actix_web::cookie::SameSite::Strict)
            .finish();

        // Remove cache entry
        cache.delete_one(filter, None).await?;

        Ok(HttpResponse::Ok()
            .cookie(cookie)
            .finish())
    } else {
        Err(AppError::InvalidOrExpiredCode)
    }
}

// Update the login endpoint to use LoginData
#[post("/auth/login")]
pub async fn login(
    client: web::Data<Client>,
    data: web::Json<LoginData>,
) -> Result<impl Responder, AppError> {
    let collection = client.database("mydb").collection::<User>("users");


    let user = match collection.find_one(doc! { "email": &data.email }, None).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AppError::UserNotFound),
        Err(e) => {
            println!("Database Find Error: {}", e);
            return Err(AppError::DatabaseError(e.to_string()));
        }
    };

    let cache = client.database("mydb").collection::<Document>("cache");
    if cache.find_one(doc! { "email": &data.email, "action": "login" }, None).await?.is_some() {
        cache.delete_one(doc! { "email": &data.email, "action": "login" }, None).await?;
    }

    // Generate random 6-digit code
    let code: u32 = rand::thread_rng().gen_range(100_000..1_000_000);

    let cache_entry = doc! {
        "username": &user.username,
        "email": &data.email,
        "code": code,
        "action": "login",
        "createdAt": bson::DateTime::from_millis(Utc::now().timestamp_millis()),
    };

    client.database("mydb").collection::<Document>("cache").insert_one(cache_entry, None).await?;

    // TODO: Send the code to the user's email

    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Verification code sent" })))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(verify_register);
    cfg.service(verify_login);
}
