mod config;
mod controllers;
mod db;
mod grpc;
mod middlewares;
mod models;
mod routes;
mod services;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Server;

use crate::routes::{auth_routes, health_routes};
use crate::grpc::{AppState, AuthServer};
use crate::db::mysql;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8081".to_string());
    let server_url = format!("{}:{}", host, port);

    let grpc_host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let grpc_port = env::var("GRPC_PORT").unwrap_or_else(|_| "50051".to_string());
    let grpc_url = format!("{}:{}", grpc_host, grpc_port);

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // MySQL 연결 풀 초기화
    let pool = mysql::create_pool()
        .await
        .expect("MySQL 데이터베이스 연결 실패");

    // 공유 상태 생성
    let app_state = Arc::new(Mutex::new(AppState {
        pool: pool.clone(),
        jwt_secret: jwt_secret.clone(),
    }));

    // gRPC 서버 설정
    info!("🚀 gRPC 서버 시작 - {}", grpc_url);
    let grpc_service = AuthServer::service(app_state.clone());
    let grpc_addr = grpc_url.parse().unwrap();
    
    // gRPC 서버 시작
    tokio::spawn(async move {
        if let Err(e) = Server::builder()
            .add_service(grpc_service)
            .serve(grpc_addr)
            .await
        {
            eprintln!("gRPC 서버 오류: {}", e);
        }
    });

    info!("🚀 인증 서비스 HTTP 서버 시작 - http://{}", server_url);

    // HTTP 서버 설정 및 시작
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone())) // MySQL 풀 주입
            .wrap(cors)
            .wrap(Logger::default())
            .configure(health_routes::configure)
            .configure(auth_routes::configure)
    })
    .bind(server_url)?
    .run()
    .await
}
