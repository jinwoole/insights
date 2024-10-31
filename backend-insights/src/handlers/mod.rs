pub mod auth_handler;
pub mod user_handler;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(auth_handler::init)
            .configure(user_handler::init),
    );
}
