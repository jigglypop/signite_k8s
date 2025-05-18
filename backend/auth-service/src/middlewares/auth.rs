use actix_web::{
    dev::Payload, error::ErrorUnauthorized, http::header, web, Error, FromRequest, HttpRequest,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use uuid::Uuid;

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,    // 사용자 ID
    pub email: String,  // 이메일
    pub name: String,   // 이름
    pub exp: i64,       // 만료 시간
    pub iat: i64,       // 발급 시간
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub id: String,
    pub email: String,
    pub name: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get(header::AUTHORIZATION);
        
        if let Some(auth_header) = auth_header {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str.trim_start_matches("Bearer ");
                    
                    if let Ok(claims) = validate_token(token) {
                        return ready(Ok(AuthenticatedUser {
                            id: claims.sub,
                            email: claims.email,
                            name: claims.name,
                        }));
                    }
                }
            }
        }
        
        ready(Err(ErrorUnauthorized("Invalid token")))
    }
}

pub fn generate_token(user_id: &str, email: &str, name: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let config = Config::init();
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(config.jwt_expires_in))
        .expect("valid timestamp")
        .timestamp();
    
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        name: name.to_string(),
        exp: expiration,
        iat: Utc::now().timestamp(),
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let config = Config::init();
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )?;
    
    Ok(token_data.claims)
} 