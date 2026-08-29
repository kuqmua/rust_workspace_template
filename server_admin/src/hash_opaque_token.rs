pub(crate) fn hash_opaque_token(
    token: &crate::admin_opaque_token::AdminOpaqueToken,
) -> Result<
    crate::admin_token_hash::AdminTokenHash,
    crate::admin_secret_text_error::AdminSecretTextError,
> {
    let digest = <sha2::Sha256 as sha2::Digest>::digest(token.expose().as_ref().as_bytes());
    let hash = base16ct::lower::encode_string(&digest);
    Ok(
        server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(hash)
            .map(crate::admin_token_hash::AdminTokenHash::new)?,
    )
}
