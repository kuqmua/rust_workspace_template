pub fn token(
    admin_opaque_token: &crate::admin_opaque_token::AdminOpaqueToken,
) -> Result<
    crate::admin_token_hash::AdminTokenHash,
    crate::admin_secret_text_error::AdminSecretTextError,
> {
    crate::hash_opaque_token::hash_opaque_token(admin_opaque_token)
}
