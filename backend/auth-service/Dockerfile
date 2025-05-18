FROM rust:1.82 as builder

WORKDIR /app

# 필요한 도구 설치
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# 의존성 캐싱을 위해 Cargo.toml 파일만 먼저 복사
COPY Cargo.toml ./

# .sqlx 폴더 복사 (이걸 먼저 해야함)
COPY .sqlx ./.sqlx/

# 더미 빌드를 통해 의존성 레이어 캐싱
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 실제 소스 코드 복사
COPY . .

# 실제 빌드 - SQLx 오프라인 모드 사용
ENV SQLX_OFFLINE=true
RUN cargo build --release

# 최종 이미지: 같은 Rust 이미지 기반으로 변경
FROM rust:1.82-slim

WORKDIR /app

# SSL 인증서와 기타 필요한 런타임 의존성 설치
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# 빌드 결과물 복사
COPY --from=builder /app/target/release/auth-service .

# 기본 환경 변수 설정
ENV PORT=8081
ENV DATABASE_URL="mysql://user:password@mysql:3306/auth_db"

EXPOSE 8081

CMD ["./auth-service"] 