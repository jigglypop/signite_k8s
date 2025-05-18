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

# 서비스 배포 (로컬 이미지 빌드 및 적용)
echo "Auth Service 빌드 및 배포..."
cd "${CURRENT_DIR}/../backend/auth-service"
docker build -t signite/auth-service:local .
kubectl apply -f "${CURRENT_DIR}/auth-service/" -n signite

echo "Sigboard Service 빌드 및 배포..."
cd "${CURRENT_DIR}/../backend/sigboard-service"
docker build -t signite/sigboard-service:local .
kubectl apply -f "${CURRENT_DIR}/sigboard-service/" -n signite

# 원래 디렉토리로 복귀
cd "${CURRENT_DIR}"

# 포트포워딩 설정
echo "포트포워딩 설정 중..."
kubectl port-forward -n signite svc/auth-service 8081:80 &
kubectl port-forward -n signite svc/sigboard-service 8082:80 &

echo "배포 완료! 서비스는 아래 URL에서 접속 가능합니다:"
echo "Auth Service: http://localhost:8081"
echo "Sigboard Service: http://localhost:8082"
echo ""
echo "카프카: 9092 포트" 