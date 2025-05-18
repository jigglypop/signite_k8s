use actix_web::{web, HttpResponse, Responder};
use log::error;
use sqlx::MySqlPool;

use crate::{
    middlewares::AuthenticatedUser,
    models::{CreateUserDto, LoginDto},
    services::auth_service,
};

pub async fn register(pool: web::Data<MySqlPool>, user_dto: web::Json<CreateUserDto>) -> impl Responder {
    match auth_service::register(pool.get_ref(), user_dto.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e })),
    }
}

pub async fn login(pool: web::Data<MySqlPool>, login_dto: web::Json<LoginDto>) -> impl Responder {
    match auth_service::login(pool.get_ref(), login_dto.into_inner()).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(e) => HttpResponse::Unauthorized().json(serde_json::json!({ "error": e })),
    }
}

pub async fn me(pool: web::Data<MySqlPool>, user: AuthenticatedUser) -> impl Responder {
    match auth_service::get_user_by_id(pool.get_ref(), &user.id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().json(serde_json::json!({ "error": e })),
    }
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
} 