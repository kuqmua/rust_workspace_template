#[derive(
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_getters::Getters,
)]
#[getters(bare, legacy_refs)]
pub struct AdminGeneratedToken {
    hash: crate::admin_token_hash::AdminTokenHash,
    token: crate::admin_opaque_token::AdminOpaqueToken,
}

impl AdminGeneratedToken {
    pub fn generate() -> Result<Self, crate::admin_secret_text_error::AdminSecretTextError> {
        let token = server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(format!(
            "{}.{}",
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4()
        ))
        .map(crate::admin_opaque_token::AdminOpaqueToken::new)?;
        crate::hash_opaque_token::hash_opaque_token(&token).map(|hash| Self::new(hash, token))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generated_token_accessors_preserve_borrowed_compatibility() {
        let secret = || {
            server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(String::from(
                constants_str::A_ALT,
            ))
            .expect(constants_str::DIAGNOSTIC_1A6AC3D2)
        };
        let admin_generated_token = super::AdminGeneratedToken::new(
            crate::admin_token_hash::AdminTokenHash::from(secret()),
            crate::admin_opaque_token::AdminOpaqueToken::from(secret()),
        );
        assert!(std::ptr::eq(
            admin_generated_token.hash(),
            admin_generated_token.get_hash()
        ));
        assert!(std::ptr::eq(
            admin_generated_token.token(),
            admin_generated_token.get_token()
        ));
    }
}
