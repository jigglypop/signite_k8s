# Signite - 쿠버네티스 배포 가이드

## 사전 준비 사항

1. AWS CLI 설치 및 설정
2. kubectl 설치 및 설정
3. AWS EKS 클러스터 생성
4. AWS ECR 레포지토리 생성 (auth-service, sigboard-service)
5. AWS RDS PostgreSQL 데이터베이스 생성 (선택사항, 프로덕션 환경 권장)

## 환경 변수 설정

```bash
# AWS ECR 계정 및 리전 설정
export AWS_REGION=ap-northeast-2
export AWS_ECR_ACCOUNT=123456789012

# SSL 인증서 ARN 설정
export AWS_CERTIFICATE_ARN=arn:aws:acm:ap-northeast-2:123456789012:certificate/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
```

## 이미지 빌드 및 푸시

```bash
# 인증 서비스 이미지 빌드 및 푸시
cd ../backend/auth-service
docker build -t $AWS_ECR_ACCOUNT.dkr.ecr.$AWS_REGION.amazonaws.com/signite/auth-service:latest .
aws ecr get-login-password --region $AWS_REGION | docker login --username AWS --password-stdin $AWS_ECR_ACCOUNT.dkr.ecr.$AWS_REGION.amazonaws.com
docker push $AWS_ECR_ACCOUNT.dkr.ecr.$AWS_REGION.amazonaws.com/signite/auth-service:latest

# 서명 서비스 이미지 빌드 및 푸시
cd ../sigboard-service
docker build -t $AWS_ECR_ACCOUNT.dkr.ecr.$AWS_REGION.amazonaws.com/signite/sigboard-service:latest .
docker push $AWS_ECR_ACCOUNT.dkr.ecr.$AWS_REGION.amazonaws.com/signite/sigboard-service:latest
```

## 쿠버네티스 배포

```bash
# 네임스페이스 생성
kubectl apply -f namespace.yaml

# 시크릿 및 ConfigMap 생성
kubectl apply -f secrets.yaml
kubectl apply -f postgres-init-configmap.yaml

# PostgreSQL 배포
kubectl apply -f postgres.yaml

# 마이크로서비스 배포
kubectl apply -f auth-service/deployment.yaml
kubectl apply -f auth-service/service.yaml
kubectl apply -f sigboard-service/deployment.yaml
kubectl apply -f sigboard-service/service.yaml

# Ingress 배포
kubectl apply -f ingress.yaml
```

## 배포 확인

```bash
# 파드 상태 확인
kubectl get pods -n signite

# 서비스 확인
kubectl get svc -n signite

# Ingress 확인
kubectl get ingress -n signite
```

## 로컬 개발 환경

로컬 개발 환경은 Docker Compose를 사용합니다:

```bash
cd ../backend
docker-compose up -d
```

## 주의사항

- 프로덕션 환경에서는 시크릿 값을 적절하게 변경해야 합니다.
- AWS RDS를 사용하는 경우 `postgres.yaml` 대신 외부 데이터베이스를 사용하도록 설정을 변경해야 합니다.
- AWS ECR 레포지토리는 사전에 생성되어 있어야 합니다.
- AWS EKS 클러스터에 AWS Load Balancer Controller가 설치되어 있어야 Ingress가 정상적으로 작동합니다. 