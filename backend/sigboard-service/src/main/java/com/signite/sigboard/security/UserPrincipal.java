package com.signite.sigboard.security;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.security.Principal;

@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
public class UserPrincipal implements Principal {
    private String id;
    private String email;
    private String name;

    @Override
    public String getName() {
        return email;
    }
} 