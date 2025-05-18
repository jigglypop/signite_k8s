use std::env;

pub struct Config {
    pub jwt_secret: String,
    pub jwt_expires_in: i64,
    pub database_url: String,
}

impl Config {
    pub fn init() -> Self {
        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your_jwt_secret_key".to_string());
        let jwt_expires_in = env::var("JWT_EXPIRES_IN")
            .unwrap_or_else(|_| "3600".to_string())
            .parse::<i64>()
            .unwrap_or(3600);
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "mysql://signite:signite@mysql:3306/auth_db".to_string());

        Self {
            jwt_secret,
            jwt_expires_in,
            database_url,
        }
    }
} 