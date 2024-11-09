//! # 인증 모듈
//! 
//! 이 모듈은 이메일 기반의 인증 기능을 제공하며, 인증 코드를 사용합니다.
//! 회원가입과 로그인 모두 2단계 인증 프로세스로 구현되어 있습니다.
//! 
//! ## 주요 기능
//! 
//! * 회원가입을 위한 2단계 이메일 인증
//! * 로그인을 위한 2단계 이메일 인증
//! * MongoDB의 TTL 인덱스를 활용한 임시 코드 저장
//! * 보안 쿠키를 통한 JWT 기반 인증
//! 
//! ## 인증 프로세스
//! 
//! ### 회원가입 과정:
//! 1. 사용자가 사용자명과 이메일 제출 (`/register`)
//! 2. 시스템이 기존 사용자명/이메일 중복 확인
//! 3. 6자리 인증 코드 생성
//! 4. 3분 유효기간으로 캐시에 코드 저장
//! 5. 사용자가 인증 코드 확인 (`/verify/register`)
//! 6. 시스템이 사용자 계정 생성 및 JWT 발급
//! 
//! ### 로그인 과정:
//! 1. 사용자가 이메일 제출 (`/login`)
//! 2. 시스템이 6자리 인증 코드 생성
//! 3. 3분 유효기간으로 캐시에 코드 저장
//! 4. 사용자가 인증 코드 확인 (`/verify/login`)
//! 5. 시스템이 보안 쿠키로 JWT 발급
//! 
//! ## 보안 기능
//! 
//! * JWT 저장을 위한 HTTPOnly 쿠키 사용
//! * 보안 쿠키 플래그 활성화
//! * Strict 정책의 same-site 쿠키 설정
//! * 3분 후 자동으로 인증 코드 만료
//! * 일회용 인증 코드 사용
//! 
//! ## 데이터 모델
//! 
//! * `RegisterData`: 회원가입용 사용자명과 이메일
//! * `LoginData`: 로그인용 이메일
//! * `VerificationRegisterData`: 회원가입 인증 데이터
//! * `VerificationLoginData`: 로그인 ���증 데이터
//! 
//! ## 사용된 크레이트
//! 
//! * `actix-web`: 웹 프레임워크
//! * `mongodb`: 데이터베이스 작업
//! * `chrono`: 타임스탬프 처리
//! * `rand`: 인증 코드 생성
//! * `serde`: 데이터 직렬화
//! 
//! ## 참고사항
//! 
//! 이메일 전송 기능은 현재 TODO 항목으로 남아있으며 구현이 필요합니다.
//! TODO 주석이 표시된 코드 섹션에 구현이 추가되어야 합니다.

use actix_web::{post, web, HttpResponse, Responder, cookie::Cookie};
use mongodb::Client;
use serde::{Deserialize, Serialize};
use mongodb::bson::{doc, Document};
use mongodb::options::IndexOptions;
use mongodb::IndexModel;
use chrono::Utc;
use rand::Rng;

// 사용자 모델과 인증 응답 모델을 가져옵니다.
use crate::models::user::User;
// 비밀번호 해시와 JWT 생성 유틸리티를 가져옵니다.
use crate::utils::jwt::generate_jwt;
use crate::errors::AppError;
use bson::oid::ObjectId;

// 회원가입 시 필요한 데이터 구조체입니다.
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub email: String,
}

// 로그인 시 필요한 데이터 구조체입니다.
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
    pub email: String,
}

// 회원가입 인증 시 필요한 데이터 구조체입니다.
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationRegisterData {
    pub username: String,
    pub email: String,
    pub code: u32,
}

// 로그인 인증 시 필요한 데이터 구조체입니다.
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationLoginData {
    pub email: String,
    pub code: u32,
}

// 첫 번째 엔드포인트: 사용자명과 이메일을 받아서 코드 생성 후 캐시에 저장합니다.
#[post("/register")]
pub async fn register(
    client: web::Data<Client>,
    ses_client: web::Data<sesClient>, // Get SES client from app data
    data: web::Json<RegisterData>,
) -> Result<impl Responder, AppError> {
    let users = client.database("insights").collection::<User>("users");
    let cache = client.database("insights").collection::<Document>("cache");
    if data.username.trim().is_empty() {
        return Err(AppError::InvalidInput("Username Error".to_string()));
    }
    
    if data.email.trim().is_empty() {
        return Err(AppError::InvalidInput("Email Error".to_string()));
    }

        // 이미 존재하는 사용자명인지 확인합니다.
    if users
        .find_one(doc! { "username": &data.username }, None)
        .await?
        .is_some()
    {
        return Err(AppError::UserAlreadyExists);
    }

    // 이미 존재하는 이메일인지 확인합니다.
    if users
        .find_one(doc! { "email": &data.email }, None)
        .await?
        .is_some()
    {
        return Err(AppError::UserAlreadyExists);
    }

    let existing = cache.find_one(doc! { "email": &data.email }, None).await?;
    if existing.is_some() {
        cache.delete_one(doc! { "email": &data.email }, None).await?;
    }

    // "createdAt" 필드에 TTL 인덱스를 생성합니다.
    let index_options = IndexOptions::builder()
        .expire_after(Some(std::time::Duration::from_secs(180)))
        .build();
    let index_model = IndexModel::builder()
        .keys(doc! { "createdAt": 1 })
        .options(index_options)
        .build();
    cache.create_index(index_model, None).await?;

    // 랜덤한 6자리 코드를 생성합니다.
    let code: u32 = rand::thread_rng().gen_range(100_000..1_000_000);

    let cache_entry = doc! {
        "username": &data.username,
        "email": &data.email,
        "code": code,
        "createdAt": bson::DateTime::from_millis(Utc::now().timestamp_millis()),
    };
    

    // Call send_message with 'to' and 'code' only
    send_message(&data.email, code, &ses_client).await.map_err(|e| AppError::EmailError(e.to_string()))?;

    cache.insert_one(cache_entry, None).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "ok" })))
}

// 회원가입 인증을 처리합니다.
#[post("/verify/register")]
pub async fn verify_register(
    client: web::Data<Client>,
    data: web::Json<VerificationRegisterData>,
) -> Result<impl Responder, AppError> {
    let cache = client.database("insights").collection::<Document>("cache");

    // 코드가 유효한지 확인합니다.
    let filter = doc! {
        "username": &data.username,
        "email": &data.email,
        "code": data.code,
    };

    let cache_entry = cache.find_one(filter.clone(), None).await?;
    if let Some(entry) = cache_entry {
        let username = entry.get_str("username").map_err(|_| AppError::InvalidOrExpiredCode)?.to_string();

        let users = client.database("insights").collection::<User>("users");

        // 새로운 사용자를 생성합니다.
        let new_user = User {
            id: ObjectId::new(),
            username,
            email: data.email.clone(),
            // 필요한 다른 필드들을 초기화합니다.
        };

        users.insert_one(&new_user, None).await?;

        // 캐시 항목을 삭제합니다.
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

// 로그인 인증을 처리합니다.
#[post("/verify/login")]
pub async fn verify_login(
    client: web::Data<Client>,
    data: web::Json<VerificationLoginData>,
) -> Result<impl Responder, AppError> {
    let cache = client.database("insights").collection::<Document>("cache");

    // 코드가 유효한지 확인합니다.
    let filter = doc! {
        "email": &data.email,
        "code": data.code,
    };

    let cache_entry = cache.find_one(filter.clone(), None).await?;
    if let Some(_entry) = cache_entry {
        let users = client.database("insights").collection::<User>("users");
        let user = users.find_one(doc! { "email": &data.email }, None).await?
            .ok_or(AppError::UserNotFound)?;

        let token = generate_jwt(&user.id.to_hex())?;
        let cookie = Cookie::build("token", token)
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(actix_web::cookie::SameSite::Strict)
            .finish();

        // 캐시 항목을 삭제합니다.
        cache.delete_one(filter, None).await?;

        Ok(HttpResponse::Ok()
            .cookie(cookie)
            .finish())
    } else {
        Err(AppError::InvalidOrExpiredCode)
    }
}

// Update the login endpoint to use LoginData
#[post("/login")]
pub async fn login(
    client: web::Data<Client>,
    ses_client: web::Data<sesClient>, // Get SES client from app data
    data: web::Json<LoginData>,
) -> Result<impl Responder, AppError> {
    let collection = client.database("insights").collection::<User>("users");

    // 사용자를 검색합니다.
    let user = match collection.find_one(doc! { "email": &data.email }, None).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AppError::UserNotFound),
        Err(e) => {
            return Err(AppError::DatabaseError(e.to_string()));
        }
    };

    let cache = client.database("insights").collection::<Document>("cache");
    if cache.find_one(doc! { "email": &data.email, "action": "login" }, None).await?.is_some() {
        cache.delete_one(doc! { "email": &data.email, "action": "login" }, None).await?;
    }

    // 랜덤한 6자리 코드를 생성합니다.
    let code: u32 = rand::thread_rng().gen_range(100_000..1_000_000);

    let cache_entry = doc! {
        "username": &user.username,
        "email": &data.email,
        "code": code,
        "action": "login",
        "createdAt": bson::DateTime::from_millis(Utc::now().timestamp_millis()),
    };

    cache.insert_one(cache_entry, None).await?;

    // Call send_message with 'to' and 'code' only
    send_message(&data.email, code, &ses_client).await.map_err(|e| AppError::EmailError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Verification code sent" })))
}

use aws_sdk_sesv2::{Client as sesClient, Error};
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};

// Email sending function
async fn send_message(
    to: &str,
    code: u32,
    ses_client: &sesClient,
) -> Result<(), Error> {
    let from = "jinwoolee42@outlook.com"; // Use a verified SES email
    let subject = "Your Verification Code";
    let message = format!("Your verification code is: {}", code);

    let dest = Destination::builder()
        .to_addresses(to)
        .build();

    let subject_content = Content::builder()
        .data(subject)
        .charset("UTF-8")
        .build()
        .expect("building Content");

    let body_content = Content::builder()
        .data(&message)
        .charset("UTF-8")
        .build()
        .expect("building Content");

    let body = Body::builder().text(body_content).build();

    let msg = Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    let email_content = EmailContent::builder().simple(msg).build();
    println!("Register data");
    ses_client
        .send_email()
        .from_email_address(from)
        .destination(dest)
        .content(email_content)
        .send()
        .await?;

    println!("Email sent to {}", to);

    Ok(())
}