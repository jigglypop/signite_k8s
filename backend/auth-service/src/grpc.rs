use crate::models::User;
use crate::db::users::UserRecord;
use tonic::{Request, Response, Status};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

// Proto에서 생성된 코드
pub mod auth {
    tonic::include_proto!("auth");
}

use auth::{
    auth_service_server::{AuthService, AuthServiceServer},
    ValidateTokenRequest, ValidateTokenResponse,
    GetUserRequest, UserResponse,
    FindUserByEmailRequest,
};

#[derive(Debug)]
pub struct AppState {
    pub pool: MySqlPool,
    pub jwt_secret: String,
}

#[derive(Debug)]
pub struct AuthServer {
    state: Arc<Mutex<AppState>>,
}

impl AuthServer {
    pub fn new(state: Arc<Mutex<AppState>>) -> Self {
        Self { state }
    }

    pub fn service(state: Arc<Mutex<AppState>>) -> AuthServiceServer<AuthServer> {
        let server = AuthServer::new(state);
        AuthServiceServer::new(server)
    }
}

#[tonic::async_trait]
impl AuthService for AuthServer {
    async fn validate_token(
        &self,
        request: Request<ValidateTokenRequest>,
    ) -> Result<Response<ValidateTokenResponse>, Status> {
        let state = self.state.lock().await;
        let req = request.into_inner();
        let token = req.token;

        // JWT 클레임 구조체 정의
        #[derive(Debug, serde::Deserialize)]
        struct Claims {
            pub sub: String,
            pub email: String,
            pub name: String,
            pub exp: i64,
        }

        // 토큰 검증
        let validation = Validation::new(Algorithm::HS256);
        let token_data = match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &validation,
        ) {
            Ok(data) => data,
            Err(_) => {
                return Ok(Response::new(ValidateTokenResponse {
                    valid: false,
                    user_id: "".to_string(),
                    email: "".to_string(),
                    name: "".to_string(),
                    expires_at: 0,
                }));
            }
        };

        let claims = token_data.claims;

        Ok(Response::new(ValidateTokenResponse {
            valid: true,
            user_id: claims.sub,
            email: claims.email,
            name: claims.name,
            expires_at: claims.exp,
        }))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let state = self.state.lock().await;
        let req = request.into_inner();
        let user_id = req.user_id;

        // UUID 변환
        let uuid = match Uuid::parse_str(&user_id) {
            Ok(id) => id,
            Err(_) => {
                return Ok(Response::new(UserResponse {
                    id: "".to_string(),
                    email: "".to_string(),
                    name: "".to_string(),
                    created_at: 0,
                    success: false,
                    error_message: "Invalid user ID format".to_string(),
                }));
            }
        };

        // 데이터베이스에서 사용자 조회
        let user = match sqlx::query_as::<_, UserRecord>(
            "SELECT id, email, name, password_hash, created_at, updated_at FROM users WHERE id = ?"
        )
        .bind(uuid.to_string())
        .fetch_optional(&state.pool)
        .await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Ok(Response::new(UserResponse {
                    id: "".to_string(),
                    email: "".to_string(),
                    name: "".to_string(),
                    created_at: 0,
                    success: false,
                    error_message: "User not found".to_string(),
                }));
            }
            Err(e) => {
                return Ok(Response::new(UserResponse {
                    id: "".to_string(),
                    email: "".to_string(),
                    name: "".to_string(),
                    created_at: 0,
                    success: false,
                    error_message: format!("Database error: {}", e),
                }));
            }
        };

        // Unix timestamp로 변환 (초 단위)
        let created_at_timestamp = user.created_at.timestamp();

        Ok(Response::new(UserResponse {
            id: user.id.to_string(),
            email: user.email,
            name: user.name,
            created_at: created_at_timestamp,
            success: true,
            error_message: "".to_string(),
        }))
    }

    async fn find_user_by_email(
        &self,
        request: Request<FindUserByEmailRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let state = self.state.lock().await;
        let req = request.into_inner();
        let email = req.email;

        // 데이터베이스에서 이메일로 사용자 조회
        let user = match sqlx::query_as::<_, UserRecord>(
            "SELECT id, email, name, password_hash, created_at, updated_at FROM users WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(&state.pool)
        .await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Ok(Response::new(UserResponse {
                    id: "".to_string(),
                    email: "".to_string(),
                    name: "".to_string(),
                    created_at: 0,
                    success: false,
                    error_message: "User not found".to_string(),
                }));
            }
            Err(e) => {
                return Ok(Response::new(UserResponse {
                    id: "".to_string(),
                    email: "".to_string(),
                    name: "".to_string(),
                    created_at: 0,
                    success: false,
                    error_message: format!("Database error: {}", e),
                }));
            }
        };

        // Unix timestamp로 변환 (초 단위)
        let created_at_timestamp = user.created_at.timestamp();

        Ok(Response::new(UserResponse {
            id: user.id.to_string(),
            email: user.email,
            name: user.name,
            created_at: created_at_timestamp,
            success: true,
            error_message: "".to_string(),
        }))
    }
} 