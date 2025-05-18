package com.signite.sigboard.repository;

import com.signite.sigboard.model.Signature;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import java.util.List;
import java.util.UUID;

@Repository
public interface SignatureRepository extends JpaRepository<Signature, UUID> {
    
    List<Signature> findByUserId(String userId);
    
    List<Signature> findByDocumentId(String documentId);
    
    Signature findByUserIdAndDocumentId(String userId, String documentId);
} 