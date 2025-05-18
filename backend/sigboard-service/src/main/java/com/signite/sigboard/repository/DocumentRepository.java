package com.signite.sigboard.repository;

import com.signite.sigboard.model.Document;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import java.util.List;
import java.util.UUID;

@Repository
public interface DocumentRepository extends JpaRepository<Document, UUID> {
    
    List<Document> findByCreatedBy(String createdBy);
    
    List<Document> findBySignedBy(String signedBy);
    
    List<Document> findByStatus(Document.DocumentStatus status);
} 