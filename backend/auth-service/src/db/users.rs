use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use chrono::{DateTime, Utc};
use log::{error, info};
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::models::{User, CreateUserDto, UserResponse};

pub async fn create_user(pool: &MySqlPool, user_dto: &CreateUserDto) -> Result<UserResponse, sqlx::Error> {
    // 이메일 중복 체크
    let row = sqlx::query("SELECT email FROM users WHERE email = ?")
        .bind(&user_dto.email)
        .fetch_optional(pool)
        .await?;

    if row.is_some() {
        return Err(sqlx::Error::RowNotFound); // 이메일 중복 시 에러 반환
    }

    // 비밀번호 해싱
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(user_dto.password.as_bytes(), &salt)
        .map_err(|_| sqlx::Error::RowNotFound)?
        .to_string();

    // 사용자 ID 생성
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    // 사용자 저장
    let result = sqlx::query(
        r#"
        INSERT INTO users (id, email, name, password_hash, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&id)
    .bind(&user_dto.email)
    .bind(&user_dto.name)
    .bind(&password_hash)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(UserResponse {
        id,
        email: user_dto.email.clone(),
        name: user_dto.name.clone(),
        created_at: now,
        updated_at: now,
    })
}

pub async fn find_user_by_email(
    pool: &MySqlPool,
    email: &str,
) -> Result<Option<UserRecord>, sqlx::Error> {
    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        SELECT id, email, name, password_hash, created_at, updated_at
        FROM users
        WHERE email = ?
        "#
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn find_user_by_id(
    pool: &MySqlPool,
    id: &str,
) -> Result<Option<UserRecord>, sqlx::Error> {
    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        SELECT id, email, name, password_hash, created_at, updated_at
        FROM users
        WHERE id = ?
        "#
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    
    Ok(user)
}

#[derive(sqlx::FromRow)]
pub struct UserRecord {
    pub id: String,
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn find_by_email(pool: &MySqlPool, email: &str) -> Result<Option<User>, String> {
    let result = sqlx::query_as::<_, UserRecord>(
        r#"
        SELECT id, email, name, password_hash, created_at, updated_at
        FROM users
        WHERE email = ?
        "#
    )
    .bind(email)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    match result {
        Some(record) => {
            let user = User {
                id: record.id,
                email: record.email,
                name: record.name,
                password_hash: record.password_hash,
                created_at: record.created_at,
                updated_at: record.updated_at,
            };
            Ok(Some(user))
        },
        None => Ok(None),
    }
}

pub async fn find_by_id(pool: &MySqlPool, id: &str) -> Result<Option<User>, String> {
    let result = sqlx::query_as::<_, UserRecord>(
        r#"
        SELECT id, email, name, password_hash, created_at, updated_at
        FROM users
        WHERE id = ?
        "#
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    match result {
        Some(record) => {
            let user = User {
                id: record.id,
                email: record.email,
                name: record.name,
                password_hash: record.password_hash,
                created_at: record.created_at,
                updated_at: record.updated_at,
            };
            Ok(Some(user))
        },
        None => Ok(None),
    }
}