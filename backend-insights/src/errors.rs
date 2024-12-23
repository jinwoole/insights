// src/errors.rs

use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    InternalError,
    UserNotFound,
    InvalidToken,
    MissingToken,
    DatabaseError(String), // 오류 메시지 포함
    HashingError,
    TokenCreationError,
    UserAlreadyExists,
    EmailError(String), // added
    InvalidOrExpiredCode, // 새로운 에러 추가
    InvalidInput(String), // New variant added
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InternalError => write!(f, "내부 서버 오류"),
            AppError::UserNotFound => write!(f, "사용자를 찾을 수 없습니다."),
            AppError::InvalidToken => write!(f, "잘못된 토큰"),
            AppError::MissingToken => write!(f, "토큰 누락"),
            AppError::DatabaseError(msg) => write!(f, "데이터베이스 오류: {}", msg),
            AppError::HashingError => write!(f, "해싱 오류"),
            AppError::TokenCreationError => write!(f, "토큰 생성 오류"),
            AppError::UserAlreadyExists => write!(f, "사용자가 이미 존재함"),
            AppError::EmailError(msg) => write!(f, "Email error: {}", msg),
            AppError::InvalidOrExpiredCode => write!(f, "유효하지 않거나 만료된 코드입니다."),
            AppError::InvalidInput(msg) => write!(f, "유효하지 않은 입력: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::UserNotFound
            | AppError::InvalidToken
            | AppError::MissingToken => {
                HttpResponse::Unauthorized().json(serde_json::json!({ "error": self.to_string() }))
            }
            AppError::DatabaseError(_msg) => {
                HttpResponse::InternalServerError().json(serde_json::json!({ "error": self.to_string() }))
            }
            AppError::UserAlreadyExists
            | AppError::EmailError(_) => {
                HttpResponse::Conflict().json(serde_json::json!({ "error": self.to_string() }))
            }
            AppError::InvalidOrExpiredCode => {
                HttpResponse::BadRequest().json(serde_json::json!({ "error": self.to_string() }))
            }
            AppError::InvalidInput(_msg) => {
                HttpResponse::BadRequest().json(serde_json::json!({ "error": self.to_string() }))
            }
            _ => HttpResponse::InternalServerError().json(serde_json::json!({ "error": "Internal Server Error" })),
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
