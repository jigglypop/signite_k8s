package com.signite.sigboard.dto;

import jakarta.validation.constraints.NotBlank;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
public class SignatureDto {
    
    @NotBlank(message = "사용자 ID는 필수입니다")
    private String userId;
    
    @NotBlank(message = "문서 ID는 필수입니다")
    private String documentId;
    
    @NotBlank(message = "서명 데이터는 필수입니다")
    private String signatureData;
} 