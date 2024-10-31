// src/handlers/auth_handler.rs

// auth_handler.rs
//
// 이 모듈은 사용자 인증과 관련된 API 핸들러를 제공합니다.
// 프론트엔드에서 사용할 수 있는 엔드포인트와 그 사용법은 다음과 같습니다:
//
// 1. 로그인
//    - 엔드포인트: **POST /api/login**
//    - 설명: 이메일과 비밀번호를 사용하여 사용자를 인증합니다.
//    - 요청 바디(JSON):
//      {
//          "email": "user@example.com",
//          "password": "password123"
//      }
//    - 응답:
//      - 성공(200 OK):
//        {
//            "token": "JWT 토큰 문자열",
//            "user": {
//                "id": 1,
//                "email": "user@example.com",
//                "name": "사용자 이름"
//            }
//        }
//      - 실패:
//        - 401 Unauthorized: 인증 실패
//        - 400 Bad Request: 잘못된 요청 형식
//
// 2. 회원가입
//    - 엔드포인트: **POST /api/register**
//    - 설명: 새로운 사용자를 등록합니다.
//    - 요청 바디(JSON):
//      {
//          "email": "user@example.com",
//          "password": "password123",
//          "name": "사용자 이름"
//      }
//    - 응답:
//      - 성공(201 Created):
//        {
//            "message": "회원가입 성공",
//            "user": {
//                "id": 1,
//                "email": "user@example.com",
//                "name": "사용자 이름"
//            }
//        }
//      - 실패:
//        - 400 Bad Request: 유효성 검사 실패 또는 이미 존재하는 이메일
//
// 3. 토큰 검증
//    - 엔드포인트: **GET /api/validate-token**
//    - 설명: 제공된 JWT 토큰의 유효성을 검사합니다.
//    - 헤더:
//      - Authorization: Bearer {JWT 토큰}
//    - 응답:
//      - 성공(200 OK):
//        {
//            "valid": true
//        }
//      - 실패:
//        - 401 Unauthorized: 유효하지 않은 토큰 또는 헤더 누락
//
// 4. 로그아웃
//    - 엔드포인트: **POST /api/logout**
//    - 설명: 서버 측에서 로그아웃 처리(필요한 경우 세션 무효화 등)
//    - 헤더:
//      - Authorization: Bearer {JWT 토큰}
//    - 응답:
//      - 성공(200 OK):
//        {
//            "message": "로그아웃 성공"
//        }
//      - 실패:
//        - 401 Unauthorized: 유효하지 않은 토큰 또는 헤더 누락
//
// 각 엔드포인트는 JWT 토큰을 통해 인증이 필요할 수 있습니다.

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
