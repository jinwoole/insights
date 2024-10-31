// src/errors.rs

use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    InternalError,
    InvalidCredentials,
    UserNotFound,
    InvalidToken,
    MissingToken,
    DatabaseError(String), // 오류 메시지 포함
    HashingError,
    TokenCreationError,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InternalError => write!(f, "Internal Server Error"),
            AppError::InvalidCredentials => write!(f, "Invalid Credentials"),
            AppError::UserNotFound => write!(f, "User Not Found"),
            AppError::InvalidToken => write!(f, "Invalid Token"),
            AppError::MissingToken => write!(f, "Missing Token"),
            AppError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            AppError::HashingError => write!(f, "Hashing Error"),
            AppError::TokenCreationError => write!(f, "Token Creation Error"),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InvalidCredentials
            | AppError::UserNotFound
            | AppError::InvalidToken
            | AppError::MissingToken => {
                HttpResponse::Unauthorized().body(self.to_string())
            }
            AppError::DatabaseError(msg) => {
                HttpResponse::InternalServerError().body(self.to_string())
            }
            _ => HttpResponse::InternalServerError().body("Internal Server Error"),
        }
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(err: mongodb::error::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(_: bcrypt::BcryptError) -> Self {
        AppError::HashingError
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        AppError::InvalidToken
    }
}

impl From<mongodb::bson::oid::Error> for AppError {
    fn from(_: mongodb::bson::oid::Error) -> Self {
        AppError::InternalError
    }
}

impl From<actix_web::Error> for AppError {
    fn from(_: actix_web::Error) -> Self {
        AppError::InternalError
    }
}
