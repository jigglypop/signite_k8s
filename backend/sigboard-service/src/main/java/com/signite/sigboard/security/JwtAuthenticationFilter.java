package com.signite.sigboard.security;

import auth.Auth;
import com.signite.sigboard.grpc.AuthServiceClient;
import jakarta.servlet.FilterChain;
import jakarta.servlet.ServletException;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.authority.SimpleGrantedAuthority;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.stereotype.Component;
import org.springframework.util.StringUtils;
import org.springframework.web.filter.OncePerRequestFilter;

import java.io.IOException;
import java.util.Collections;

@Component
@RequiredArgsConstructor
@Slf4j
public class JwtAuthenticationFilter extends OncePerRequestFilter {

    private final AuthServiceClient authServiceClient;

    @Override
    protected void doFilterInternal(
            HttpServletRequest request,
            HttpServletResponse response,
            FilterChain filterChain
    ) throws ServletException, IOException {
        String token = extractTokenFromRequest(request);

        if (StringUtils.hasText(token)) {
            try {
                Auth.ValidateTokenResponse validationResult = authServiceClient.validateToken(token);

                if (validationResult.getValid()) {
                    setAuthenticationContext(validationResult);
                    log.debug("사용자 인증 성공: {}", validationResult.getEmail());
                } else {
                    log.debug("토큰 유효성 검증 실패");
                }
            } catch (Exception e) {
                log.error("토큰 검증 중 오류 발생: {}", e.getMessage());
            }
        }

        filterChain.doFilter(request, response);
    }

    private String extractTokenFromRequest(HttpServletRequest request) {
        String bearerToken = request.getHeader("Authorization");
        if (StringUtils.hasText(bearerToken) && bearerToken.startsWith("Bearer ")) {
            return bearerToken.substring(7);
        }
        return null;
    }

    private void setAuthenticationContext(Auth.ValidateTokenResponse validationResult) {
        UserPrincipal userPrincipal = UserPrincipal.builder()
                .id(validationResult.getUserId())
                .email(validationResult.getEmail())
                .name(validationResult.getName())
                .build();

        UsernamePasswordAuthenticationToken authentication = new UsernamePasswordAuthenticationToken(
                userPrincipal,
                null,
                Collections.singletonList(new SimpleGrantedAuthority("ROLE_USER"))
        );

        SecurityContextHolder.getContext().setAuthentication(authentication);
    }
} 