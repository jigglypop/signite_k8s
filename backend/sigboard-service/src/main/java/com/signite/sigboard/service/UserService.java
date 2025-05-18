package com.signite.sigboard.service;

import auth.Auth;
import com.signite.sigboard.grpc.AuthServiceClient;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.stereotype.Service;

@Service
@RequiredArgsConstructor
@Slf4j
public class UserService {

    private final AuthServiceClient authServiceClient;

    /**
     * 현재 인증된 사용자의 ID를 반환
     */
    public String getCurrentUserId() {
        if (SecurityContextHolder.getContext().getAuthentication() != null &&
                SecurityContextHolder.getContext().getAuthentication().getPrincipal() instanceof com.signite.sigboard.security.UserPrincipal) {
            return ((com.signite.sigboard.security.UserPrincipal) SecurityContextHolder.getContext().getAuthentication().getPrincipal()).getId();
        }
        return null;
    }

    /**
     * 인증 서비스에서 사용자 정보 조회
     */
    public Auth.UserResponse getUserFromAuthService(String userId) {
        return authServiceClient.getUser(userId);
    }

    /**
     * 이메일로 사용자 정보 조회
     */
    public Auth.UserResponse findUserByEmail(String email) {
        return authServiceClient.findUserByEmail(email);
    }
} 