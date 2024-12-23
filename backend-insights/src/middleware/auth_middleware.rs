
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
            // HTTP-Only 쿠키에서 "token" 추출
            if let Some(cookie) = req.cookie("token") {
                let token = cookie.value();
                // JWT 토큰 검증
                match verify_jwt(token) {
                    Ok(user_id) => {
                        // 인증된 사용자 정보 삽입
                        req.extensions_mut().insert(AuthorizedUser { id: user_id });
                        // 다음 서비스 호출
                        service.call(req).await
                    }
                    Err(_) => Err(AppError::InvalidToken.into()),
                }
            } else {
                // 토큰이 없을 경우 에러 반환
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
