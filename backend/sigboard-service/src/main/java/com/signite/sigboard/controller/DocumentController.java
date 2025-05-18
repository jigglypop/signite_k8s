package com.signite.sigboard.controller;

import com.signite.sigboard.dto.DocumentDto;
import com.signite.sigboard.model.Document;
import com.signite.sigboard.service.DocumentService;
import jakarta.validation.Valid;
import lombok.RequiredArgsConstructor;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.UUID;

@RestController
@RequestMapping("/documents")
@RequiredArgsConstructor
public class DocumentController {
    
    private final DocumentService documentService;
    
    @PostMapping
    public ResponseEntity<Document> createDocument(@Valid @RequestBody DocumentDto documentDto) {
        Document document = documentService.createDocument(documentDto);
        return new ResponseEntity<>(document, HttpStatus.CREATED);
    }
    
    @GetMapping("/{id}")
    public ResponseEntity<Document> getDocument(@PathVariable UUID id) {
        Document document = documentService.getDocument(id);
        return ResponseEntity.ok(document);
    }
    
    @GetMapping("/creator/{createdBy}")
    public ResponseEntity<List<Document>> getDocumentsByCreator(@PathVariable String createdBy) {
        List<Document> documents = documentService.getDocumentsByCreator(createdBy);
        return ResponseEntity.ok(documents);
    }
    
    @GetMapping("/signer/{signedBy}")
    public ResponseEntity<List<Document>> getDocumentsBySigner(@PathVariable String signedBy) {
        List<Document> documents = documentService.getDocumentsBySigner(signedBy);
        return ResponseEntity.ok(documents);
    }
    
    @GetMapping("/status/{status}")
    public ResponseEntity<List<Document>> getDocumentsByStatus(@PathVariable Document.DocumentStatus status) {
        List<Document> documents = documentService.getDocumentsByStatus(status);
        return ResponseEntity.ok(documents);
    }
    
    @GetMapping
    public ResponseEntity<List<Document>> getAllDocuments() {
        List<Document> documents = documentService.getAllDocuments();
        return ResponseEntity.ok(documents);
    }
} 