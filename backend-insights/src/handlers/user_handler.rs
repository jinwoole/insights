// src/handlers/user_handler.rs

// user_handler.rs
//
// 이 모듈은 사용자와 관련된 API 핸들러를 제공합니다.
// 프론트엔드에서 사용할 수 있는 엔드포인트와 그 사용법은 다음과 같습니다:
//
// 1. 사용자 정보 조회
//    - 엔드포인트: **GET /api/user**
//    - 설명: 인증된 사용자의 정보를 조회합니다.
//    - 헤더:
//      - Authorization: Bearer {JWT 토큰}
//    - 응답:
//      - 성공(200 OK):
//        {
//            "id": "사용자 ID",
//            "username": "사용자 이름"
//        }
//      - 실패:
//        - 401 Unauthorized: 토큰이 유효하지 않거나 헤더가 누락된 경우
//        - 404 Not Found: 사용자를 찾을 수 없는 경우
//
// 2. 사용자 정보 업데이트
//    - 엔드포인트: **PUT /api/user**
//    - 설명: 인증된 사용자의 정보를 업데이트합니다.
//    - 헤더:
//      - Authorization: Bearer {JWT 토큰}
//    - 요청 바디(JSON):
//      {
//          "username": "새로운 사용자 이름",
//          "password": "새로운 비밀번호" // 선택 사항
//      }
//    - 응답:
//      - 성공(200 OK):
//        {
//            "message": "사용자 정보가 업데이트되었습니다."
//        }
//      - 실패:
//        - 400 Bad Request: 잘못된 요청 형식 또는 유효성 검사 실패
//        - 401 Unauthorized: 토큰이 유효하지 않거나 헤더가 누락된 경우
//        - 500 Internal Server Error: 서버 오류 발생
//
// 각 엔드포인트는 JWT 토큰을 통한 인증이 필요합니다.

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
