// src/utils/jwt.rs

use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::env;
use crate::errors::AppError;
use chrono::{Utc, Duration};
use bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // 사용자 ID (Hex 문자열)
    exp: usize,  // 만료 시간
}

pub fn generate_jwt(user_id: &str) -> Result<String, AppError> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(12))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|_e| AppError::TokenCreationError)
}

pub fn verify_jwt(token: &str) -> Result<ObjectId, AppError> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    ).map_err(|_e| AppError::InvalidToken)?;

    let user_id_str = token_data.claims.sub;
    ObjectId::parse_str(&user_id_str).map_err(|_e| AppError::InvalidToken)
}
