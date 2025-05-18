use actix_web::web;
use crate::controllers::auth_controller;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("", web::get().to(auth_controller::health_check))
    );
} 