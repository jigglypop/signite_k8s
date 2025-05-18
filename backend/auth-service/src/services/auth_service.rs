use argon2::{
    password_hash::{Error as PasswordHashError, PasswordHash, PasswordVerifier},
    Argon2,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::MySqlPool;
use std::env;
use uuid::Uuid;
use chrono::{Utc, Duration};
use log::{error, info};

use crate::{
    db::users,
    middlewares::generate_token,
    models::{CreateUserDto, LoginDto, TokenResponse, UserResponse, User},
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,      // 사용자 ID
    email: String,    // 이메일
    name: String,     // 이름
    exp: usize,       // 만료 시간 (초 단위의 타임스탬프)
    iat: usize,       // 발급 시간
}

pub async fn register(pool: &MySqlPool, user_dto: CreateUserDto) -> Result<UserResponse, String> {
    let user = users::create_user(pool, &user_dto).await
        .map_err(|e| format!("Failed to create user: {}", e))?;
    
    Ok(UserResponse {
        id: user.id,
        email: user.email,
        name: user.name,
        created_at: user.created_at,
        updated_at: user.updated_at,
    })
}

pub async fn login(pool: &MySqlPool, login_dto: LoginDto) -> Result<TokenResponse, String> {
    let user_opt = users::find_by_email(pool, &login_dto.email).await?;
    
    let user = match user_opt {
        Some(u) => u,
        None => return Err("Invalid email or password".to_string()),
    };
    
    // 비밀번호 검증
    let password_hash = PasswordHash::new(&user.password_hash)
        .map_err(|e| format!("Invalid password hash: {}", e))?;
    
    let argon2 = Argon2::default();
    
    if argon2
        .verify_password(login_dto.password.as_bytes(), &password_hash)
        .is_err()
    {
        return Err("Invalid email or password".to_string());
    }
    
    // JWT 토큰 생성
    let token = generate_token(&user.id, &user.email, &user.name)
        .map_err(|e| format!("Failed to generate token: {}", e))?;
    
    Ok(TokenResponse {
        token,
        user_id: user.id,
        email: user.email,
        name: user.name,
    })
}

pub async fn get_user_by_id(pool: &MySqlPool, user_id: &str) -> Result<UserResponse, String> {
    let user_opt = users::find_by_id(pool, user_id).await?;
    
    match user_opt {
        Some(user) => Ok(UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }),
        None => Err("User not found".to_string()),
    }
} 