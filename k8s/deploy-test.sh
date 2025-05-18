#!/bin/bash

# 네임스페이스 생성
kubectl apply -f namespace.yaml

# 시크릿 적용
kubectl apply -f secrets.yaml

# MySQL 설정 및 배포
kubectl apply -f mysql-init-configmap.yaml
kubectl apply -f mysql.yaml

# Kafka 및 Zookeeper 배포
kubectl apply -f kafka.yaml

# 현재 작업 디렉토리 저장
CURRENT_DIR=$(pwd)

# 기존 배포 삭제
kubectl delete deployment auth-service sigboard-service -n signite || true

# Auth Service 실제 이미지 빌드 및 배포
echo "Auth Service 이미지 빌드 및 배포..."
cd "${CURRENT_DIR}/../backend/auth-service"

# Proto 파일 복사
mkdir -p protos
cp "${CURRENT_DIR}/../backend/protos/auth.proto" protos/

# 이미지 빌드
docker build -t signite/auth-service-dev:latest .

# Sigboard Service 실제 이미지 빌드 및 배포
echo "Sigboard Service 이미지 빌드 및 배포..."
cd "${CURRENT_DIR}/../backend/sigboard-service"
docker build -t signite/sigboard-service:latest .

# 서비스 배포
cd "${CURRENT_DIR}"
kubectl apply -f auth-service-test.yaml
kubectl apply -f sigboard-service-test.yaml

# 포트포워딩 설정
echo "포트포워딩 설정 중..."
kubectl port-forward -n signite svc/auth-service 8081:80 &
kubectl port-forward -n signite svc/sigboard-service 8082:80 &

echo "배포 완료! 서비스는 아래 URL에서 접속 가능합니다:"
echo "Auth Service: http://localhost:8081"
echo "Sigboard Service: http://localhost:8082" 