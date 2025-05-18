package com.signite.sigboard.controller;

import com.signite.sigboard.dto.SignatureDto;
import com.signite.sigboard.model.Signature;
import com.signite.sigboard.service.SignatureService;
import jakarta.validation.Valid;
import lombok.RequiredArgsConstructor;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.UUID;

@RestController
@RequestMapping("/signatures")
@RequiredArgsConstructor
public class SignatureController {
    
    private final SignatureService signatureService;
    
    @PostMapping
    public ResponseEntity<Signature> createSignature(@Valid @RequestBody SignatureDto signatureDto) {
        Signature signature = signatureService.createSignature(signatureDto);
        return new ResponseEntity<>(signature, HttpStatus.CREATED);
    }
    
    @GetMapping("/{id}")
    public ResponseEntity<Signature> getSignature(@PathVariable UUID id) {
        Signature signature = signatureService.getSignature(id);
        return ResponseEntity.ok(signature);
    }
    
    @GetMapping("/user/{userId}")
    public ResponseEntity<List<Signature>> getSignaturesByUser(@PathVariable String userId) {
        List<Signature> signatures = signatureService.getSignaturesByUserId(userId);
        return ResponseEntity.ok(signatures);
    }
    
    @GetMapping("/document/{documentId}")
    public ResponseEntity<List<Signature>> getSignaturesByDocument(@PathVariable String documentId) {
        List<Signature> signatures = signatureService.getSignaturesByDocumentId(documentId);
        return ResponseEntity.ok(signatures);
    }
} 