// src/middleware/auth_middleware.rs
//
// 이 모듈은 JWT 토큰을 이용한 인증 미들웨어를 제공합니다.
// 프론트엔드에서 API 요청 시 인증이 필요한 엔드포인트에 대해 이 미들웨어를 적용하여
// 사용자의 인증 상태를 확인합니다.
//
// ### 미들웨어 기능:
// - 모든 요청에 대해 JWT 토큰의 유효성을 검사합니다.
// - 유효한 토큰이 없을 경우 401 Unauthorized 에러를 반환합니다.
// - 유효한 토큰이 있을 경우 요청을 다음 서비스로 전달합니다.
//
// ### 프론트엔드 사용 방법:
// - 인증이 필요한 API 요청 시 HTTP 헤더에 JWT 토큰을 포함해야 합니다.
//   - 헤더 이름: `Authorization`
//   - 형식: `Authorization: Bearer {JWT 토큰}`
// - 토큰이 만료되었거나 유효하지 않은 경우 재로그인이 필요할 수 있습니다.
//
// ### 예시:
// ```http
// GET /api/protected-endpoint HTTP/1.1
// Host: example.com
// Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ...
// ```
//
// ### 참고 사항:
// - 이 미들웨어는 서버의 모든 보호된 엔드포인트에 적용됩니다.
// - 프론트엔드에서는 토큰 관리(저장, 갱신, 삭제)를 적절히 구현해야 합니다.
// - 토큰이 없는 상태로 보호된 엔드포인트에 접근하면 401 에러가 발생합니다.
//
// 이 미들웨어를 통해 백엔드 서버는 인증된 사용자만이 특정 자원에 접근하도록 보호합니다.
// 프론트엔드에서는 사용자 로그인 후 받은 JWT 토큰을 지속적으로 관리하고 요청마다 헤더에 포함시켜야 합니다.


use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, FromRequest, HttpMessage, error::ErrorUnauthorized,
};
use futures::future::{LocalBoxFuture, ready, Ready};
use bson::oid::ObjectId;
use crate::utils::jwt::verify_jwt;
use crate::errors::AppError;
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        Box::pin(async move {
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = auth_str.trim_start_matches("Bearer ").trim();
                        match verify_jwt(token) {
                            Ok(user_id) => {
                                req.extensions_mut().insert(AuthorizedUser { id: user_id });
                                service.call(req).await
                            }
                            Err(_) => Err(AppError::InvalidToken.into()),
                        }
                    } else {
                        Err(AppError::InvalidToken.into())
                    }
                } else {
                    Err(AppError::InvalidToken.into())
                }
            } else {
                Err(AppError::MissingToken.into())
            }
        })
    }
}

#[derive(Clone)]
pub struct AuthorizedUser {
    pub id: ObjectId,
}

// FromRequest 트레이트 구현
impl FromRequest for AuthorizedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(user) = req.extensions().get::<AuthorizedUser>() {
            ready(Ok(user.clone()))
        } else {
            ready(Err(ErrorUnauthorized("Unauthorized")))
        }
    }
}
