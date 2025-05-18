package com.signite.sigboard.grpc;

import auth.Auth;
import auth.AuthServiceGrpc;
import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;
import io.grpc.StatusRuntimeException;
import jakarta.annotation.PostConstruct;
import jakarta.annotation.PreDestroy;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;

import java.util.concurrent.TimeUnit;

@Service
@Slf4j
public class AuthServiceClient {

    @Value("${auth-service.grpc.host}")
    private String host;

    @Value("${auth-service.grpc.port}")
    private int port;

    private ManagedChannel channel;
    private AuthServiceGrpc.AuthServiceBlockingStub blockingStub;

    @PostConstruct
    public void init() {
        channel = ManagedChannelBuilder.forAddress(host, port)
                .usePlaintext() // 개발 환경에서만 사용 (프로덕션에서는 SSL/TLS 사용)
                .build();
        blockingStub = AuthServiceGrpc.newBlockingStub(channel);
        log.info("gRPC 클라이언트 초기화: {}:{}", host, port);
    }

    @PreDestroy
    public void shutdown() {
        try {
            channel.shutdown().awaitTermination(5, TimeUnit.SECONDS);
        } catch (InterruptedException e) {
            log.error("gRPC 클라이언트 종료 중 오류: {}", e.getMessage());
        }
    }

    /**
     * 토큰 유효성 검증
     * @param token JWT 토큰
     * @return 토큰 검증 결과
     */
    public Auth.ValidateTokenResponse validateToken(String token) {
        Auth.ValidateTokenRequest request = Auth.ValidateTokenRequest.newBuilder()
                .setToken(token)
                .build();
        try {
            return blockingStub.validateToken(request);
        } catch (StatusRuntimeException e) {
            log.error("토큰 검증 중 RPC 실패: {}", e.getStatus());
            throw e;
        }
    }

    /**
     * 사용자 정보 조회
     * @param userId 사용자 ID
     * @return 사용자 정보
     */
    public Auth.UserResponse getUser(String userId) {
        Auth.GetUserRequest request = Auth.GetUserRequest.newBuilder()
                .setUserId(userId)
                .build();
        try {
            return blockingStub.getUser(request);
        } catch (StatusRuntimeException e) {
            log.error("사용자 조회 중 RPC 실패: {}", e.getStatus());
            throw e;
        }
    }

    /**
     * 이메일로 사용자 검색
     * @param email 이메일
     * @return 사용자 정보
     */
    public Auth.UserResponse findUserByEmail(String email) {
        Auth.FindUserByEmailRequest request = Auth.FindUserByEmailRequest.newBuilder()
                .setEmail(email)
                .build();
        try {
            return blockingStub.findUserByEmail(request);
        } catch (StatusRuntimeException e) {
            log.error("이메일로 사용자 검색 중 RPC 실패: {}", e.getStatus());
            throw e;
        }
    }
} 