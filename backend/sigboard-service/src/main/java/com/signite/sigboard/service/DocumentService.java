package com.signite.sigboard.service;

import com.signite.sigboard.dto.DocumentDto;
import com.signite.sigboard.model.Document;
import com.signite.sigboard.repository.DocumentRepository;
import jakarta.persistence.EntityNotFoundException;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.List;
import java.util.UUID;

@Service
@RequiredArgsConstructor
public class DocumentService {
    
    private final DocumentRepository documentRepository;
    private final UserService userService;
    
    @Transactional
    public Document createDocument(DocumentDto documentDto) {
        // 인증 서비스에서 사용자 정보 검증
        var userResponse = userService.findUserByEmail(documentDto.getCreatedBy());
        if (!userResponse.getSuccess()) {
            throw new IllegalArgumentException("작성자를 찾을 수 없습니다: " + documentDto.getCreatedBy());
        }
        
        // Document 객체 생성 및 저장
        Document document = Document.builder()
                .title(documentDto.getTitle())
                .content(documentDto.getContent())
                .createdBy(documentDto.getCreatedBy())
                .status(Document.DocumentStatus.PENDING)
                .build();
        
        return documentRepository.save(document);
    }
    
    public Document getDocument(UUID id) {
        return documentRepository.findById(id)
                .orElseThrow(() -> new EntityNotFoundException("문서를 찾을 수 없습니다"));
    }
    
    public List<Document> getDocumentsByCreator(String createdBy) {
        // 인증 서비스에서 사용자 정보 검증
        var userResponse = userService.findUserByEmail(createdBy);
        if (!userResponse.getSuccess()) {
            throw new IllegalArgumentException("작성자를 찾을 수 없습니다: " + createdBy);
        }
        
        return documentRepository.findByCreatedBy(createdBy);
    }
    
    public List<Document> getDocumentsBySigner(String signedBy) {
        // 인증 서비스에서 사용자 정보 검증
        var userResponse = userService.findUserByEmail(signedBy);
        if (!userResponse.getSuccess()) {
            throw new IllegalArgumentException("서명자를 찾을 수 없습니다: " + signedBy);
        }
        
        return documentRepository.findBySignedBy(signedBy);
    }
    
    public List<Document> getDocumentsByStatus(Document.DocumentStatus status) {
        return documentRepository.findByStatus(status);
    }
    
    public List<Document> getAllDocuments() {
        // 현재 사용자의 권한 확인
        String currentUserId = userService.getCurrentUserId();
        if (currentUserId == null) {
            throw new SecurityException("인증되지 않은 사용자입니다");
        }
        
        return documentRepository.findAll();
    }
} 