pub mod auth_handler;
pub mod user_handler;

use actix_web::web;
use auth_handler::{login, register, verify_login, verify_register};
use user_handler::get_user;
use crate::middleware::log_middleware::LogMiddleware;
use crate::middleware::auth_middleware::AuthMiddleware;




pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                // 인증이 필요한 엔드포인트들
                web::scope("/user")
                    .service(get_user)
                    .wrap(AuthMiddleware)
            )
            .service(
                // 인증이 필요없는 엔드포인트들
                web::scope("/auth")
                    .service(register)
                    .service(login)
                    .service(verify_register)
                    .service(verify_login)
            )
            // 모든 라우트에 로깅 미들웨어 적용
            .wrap(LogMiddleware)
    );
}
