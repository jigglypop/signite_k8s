use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;
use log::info;

pub async fn create_pool() -> Result<MySqlPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    info!("MySQL 데이터베이스에 연결 중: {}", database_url);
    
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;
    
    // 데이터베이스 연결 검증
    let result = sqlx::query("SELECT 1")
        .execute(&pool)
        .await;
    
    match result {
        Ok(_) => info!("MySQL 데이터베이스 연결 성공!"),
        Err(e) => info!("MySQL 연결 검증 실패: {}", e),
    }
    
    // 테이블 생성 확인
    ensure_tables_exist(&pool).await?;
    
    Ok(pool)
}

async fn ensure_tables_exist(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    info!("필요한 테이블 존재 여부 확인 중...");
    
    // users 테이블 생성
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id VARCHAR(36) PRIMARY KEY,
            email VARCHAR(255) NOT NULL UNIQUE,
            name VARCHAR(255) NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;
    
    info!("테이블 확인 완료");
    
    Ok(())
} 