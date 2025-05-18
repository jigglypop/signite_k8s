package com.signite.sigboard.service;

import com.signite.sigboard.dto.SignatureDto;
import com.signite.sigboard.model.Document;
import com.signite.sigboard.model.Signature;
import com.signite.sigboard.repository.DocumentRepository;
import com.signite.sigboard.repository.SignatureRepository;
import jakarta.persistence.EntityNotFoundException;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.time.LocalDateTime;
import java.util.List;
import java.util.UUID;

@Service
@RequiredArgsConstructor
public class SignatureService {
    
    private final SignatureRepository signatureRepository;
    private final DocumentRepository documentRepository;
    private final UserService userService;
    
    @Transactional
    public Signature createSignature(SignatureDto signatureDto) {
        // 사용자 검증
        var userResponse = userService.getUserFromAuthService(signatureDto.getUserId());
        if (!userResponse.getSuccess()) {
            throw new IllegalArgumentException("사용자를 찾을 수 없습니다: " + signatureDto.getUserId());
        }
        
        // 문서 조회
        Document document = documentRepository.findById(UUID.fromString(signatureDto.getDocumentId()))
                .orElseThrow(() -> new EntityNotFoundException("문서를 찾을 수 없습니다"));
        
        if (document.getStatus() != Document.DocumentStatus.PENDING) {
            throw new IllegalStateException("서명 가능한 상태가 아닙니다");
        }
        
        // 서명 생성
        Signature signature = Signature.builder()
                .userId(signatureDto.getUserId())
                .documentId(signatureDto.getDocumentId())
                .signatureData(signatureDto.getSignatureData())
                .build();
        
        // 문서 상태 업데이트
        document.setStatus(Document.DocumentStatus.SIGNED);
        document.setSignedBy(signatureDto.getUserId());
        document.setSignedAt(LocalDateTime.now());
        documentRepository.save(document);
        
        return signatureRepository.save(signature);
    }
    
    public List<Signature> getSignaturesByUserId(String userId) {
        // 사용자 검증
        var userResponse = userService.getUserFromAuthService(userId);
        if (!userResponse.getSuccess()) {
            throw new IllegalArgumentException("사용자를 찾을 수 없습니다: " + userId);
        }
        
        return signatureRepository.findByUserId(userId);
    }
    
    public List<Signature> getSignaturesByDocumentId(String documentId) {
        try {
            UUID.fromString(documentId); // 유효한 UUID 형식인지 확인
        } catch (IllegalArgumentException e) {
            throw new IllegalArgumentException("유효하지 않은a 문서 ID: " + documentId);
        }
        
        return signatureRepository.findByDocumentId(documentId);
    }
    
    public Signature getSignature(UUID id) {
        return signatureRepository.findById(id)
                .orElseThrow(() -> new EntityNotFoundException("서명을 찾을 수 없습니다"));
    }
} 