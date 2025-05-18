FROM gradle:7.6.1-jdk17 as builder

WORKDIR /app

# 의존성 캐싱을 위해 빌드 파일만 먼저 복사
COPY build.gradle ./

# 기본 settings.gradle 생성
RUN echo "rootProject.name = 'sigboard-service'" > settings.gradle

# 더미 파일 생성하여 의존성 다운로드
RUN mkdir -p src/main/java && \
    echo "public class Dummy {}" > src/main/java/Dummy.java && \
    gradle build --no-daemon || true

# 소스 코드 복사 및 빌드
COPY . .
RUN gradle build --no-daemon -x test

# 최종 이미지
FROM eclipse-temurin:17-jdk-alpine

WORKDIR /app

# 빌드된 jar 파일 복사
COPY --from=builder /app/build/libs/*.jar app.jar

EXPOSE 8082

ENTRYPOINT ["java", "-jar", "app.jar"] 