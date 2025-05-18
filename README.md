# 시그나이트(Signite) 프로젝트

## 프로젝트 개요

시그나이트는 마이크로서비스 아키텍처를 기반으로 한 웹 애플리케이션으로, 다양한 백엔드 서비스와 프론트엔드 애플리케이션을 포함하고 있습니다. 이 프로젝트는 Kubernetes 환경에서 동작하며, MySQL, Kafka 등의 인프라 서비스를 활용합니다.

## 시스템 아키텍처

시그나이트는 다음과 같은 구성요소로 이루어져 있습니다:

### 백엔드 서비스

1. **인증 서비스 (auth-service)**
   - 언어/프레임워크: Rust + Actix Web
   - 주요 기능: 사용자 인증, 회원가입, 로그인, JWT 토큰 관리
   - 데이터 저장소: MySQL

2. **게시판 서비스 (sigboard-service)**
   - 언어/프레임워크: Java + Spring Boot
   - 주요 기능: 게시판 관리, 게시글 CRUD 작업
   - 데이터 저장소: MySQL

### 프론트엔드 애플리케이션

1. **인증 앱 (auth)**
   - 언어/프레임워크: JavaScript/TypeScript + React
   - 주요 기능: 사용자 로그인, 회원가입 UI

2. **게시판 앱 (sigboard)**
   - 언어/프레임워크: JavaScript/TypeScript + React/Svelte
   - 주요 기능: 게시판 인터페이스, 게시글 작성 및 관리

3. **호스트 앱 (host)**
   - 주요 기능: 마이크로 프론트엔드 통합 및 라우팅 관리

### 인프라 구성요소

- **MySQL**: 관계형 데이터베이스로 사용자 정보와 게시글 데이터 저장
- **Kafka & Zookeeper**: 마이크로서비스 간 이벤트 기반 통신
- **Kubernetes**: 컨테이너 오케스트레이션 및 서비스 관리
- **Docker**: 애플리케이션 컨테이너화

## 시작하기

### 필수 요구사항

- Docker
- Kubernetes (minikube, kind, Docker Desktop의 Kubernetes 등)
- kubectl
- Node.js 및 npm/yarn (프론트엔드 개발용)
- Rust (auth-service 개발용)
- Java 및 Gradle (sigboard-service 개발용)

### 로컬 개발 환경 설정

1. **저장소 클론**

```bash
git clone <repository-url> signite
cd signite
```

2. **Kubernetes 네임스페이스 생성**

```bash
kubectl apply -f k8s/namespace.yaml
```

3. **MySQL 및 Kafka 배포**

```bash
kubectl apply -f k8s/mysql.yaml
kubectl apply -f k8s/kafka.yaml
```

4. **인증 서비스 빌드 및 배포**

```bash
cd backend/auth-service
docker build -t signite/auth-service-dev:latest .
kubectl apply -f ../../k8s/auth-service-test.yaml
```

5. **게시판 서비스 빌드 및 배포**

```bash
cd ../sigboard-service
docker build -t signite/sigboard-service-dev:latest .
kubectl apply -f ../../k8s/sigboard-service-test.yaml
```

6. **프론트엔드 앱 실행**

```bash
cd ../../frontend/auth
npm install
npm start
```

```bash
cd ../sigboard
npm install
npm start
```

```bash
cd ../host
npm install
npm start
```

### 포트 포워딩 설정

로컬 환경에서 서비스에 접근하기 위해 Kubernetes 포트 포워딩을 설정합니다:

```bash
kubectl port-forward -n signite svc/auth-service 8081:80
kubectl port-forward -n signite svc/sigboard-service 8082:80
kubectl port-forward -n signite svc/mysql 13306:3306
```

## API 문서

### 인증 서비스 API

#### 회원가입
- **경로**: `/auth/register`
- **메서드**: POST
- **요청 본문**:
  ```json
  {
    "email": "user@example.com",
    "name": "사용자명",
    "password": "비밀번호"
  }
  ```
- **응답**: 생성된 사용자 정보

#### 로그인
- **경로**: `/auth/login`
- **메서드**: POST
- **요청 본문**:
  ```json
  {
    "email": "user@example.com",
    "password": "비밀번호"
  }
  ```
- **응답**: JWT 토큰 및 사용자 정보

#### 사용자 정보 조회
- **경로**: `/auth/me`
- **메서드**: GET
- **헤더**: `Authorization: Bearer {jwt_token}`
- **응답**: 인증된 사용자 정보

### 게시판 서비스 API

[게시판 서비스 API 문서 추가 예정]

## 트러블슈팅

### 일반적인 이슈

1. **포트 충돌 문제**
   
   이미 사용 중인 포트로 인해 포트 포워딩 오류가 발생할 수 있습니다.
   ```bash
   # 사용 중인 포트 확인
   netstat -ano | findstr :8081
   
   # 해당 프로세스 종료 (Windows)
   taskkill /PID <process_id> /F
   
   # 또는 Linux/Mac에서
   kill -9 <process_id>
   ```

2. **MySQL 연결 오류**
   
   데이터베이스 연결 문제 발생 시:
   ```bash
   # MySQL 파드 로그 확인
   kubectl logs -n signite -l app=mysql
   
   # MySQL 서비스 포트 포워딩 확인
   kubectl port-forward -n signite svc/mysql 13306:3306
   ```

3. **Kubernetes 파드 상태 확인**
   
   ```bash
   # 파드 상태 확인
   kubectl get pods -n signite
   
   # 특정 파드 로그 확인
   kubectl logs -n signite <pod-name>
   
   # 파드 세부 정보 확인
   kubectl describe pod -n signite <pod-name>
   ```

4. **Docker 이미지 빌드 문제**

   Rust 서비스에서 sqlx 매크로 관련 이슈:
   ```bash
   # Dockerfile에서 SQLX_OFFLINE=true 환경 변수 설정 확인
   # 또는 실행 시 명시적으로 환경 변수 설정
   ENV SQLX_OFFLINE=true
   ```

## 개발자 가이드

### 코드 스타일 및 컨벤션

- **Rust**: Rust 표준 컨벤션을 따릅니다. rustfmt 및 clippy 사용 권장.
- **Java**: Google Java Style Guide를 따릅니다.
- **JavaScript/TypeScript**: ESLint, Prettier를 사용한 코드 포맷팅 권장.

### Git 워크플로우

- 기능 개발은 `feature/` 접두사를 가진 브랜치에서 진행합니다.
- 버그 수정은 `fix/` 접두사를 가진 브랜치에서 진행합니다.
- PR(Pull Request)을 통해 코드 리뷰 후 메인 브랜치에 병합합니다.

## 라이센스


## 연락처
