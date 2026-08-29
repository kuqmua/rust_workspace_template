pub(crate) fn authorization_hash_refresh_token_with_context(
    token: &crate::admin_opaque_token::AdminOpaqueToken,
    context_hash: &crate::admin_token_hash::AdminTokenHash,
) -> Result<
    crate::admin_token_hash::AdminTokenHash,
    crate::admin_secret_text_error::AdminSecretTextError,
> {
    let token_text = token.expose();
    let context_hash_text = context_hash.expose();
    let mut token_with_context = String::with_capacity(
        token_text
            .as_ref()
            .len()
            .saturating_add(context_hash_text.as_ref().len()),
    );
    token_with_context.push_str(token_text.as_ref());
    token_with_context.push_str(context_hash_text.as_ref());
    let combined_token =
        server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(token_with_context)
            .map(crate::admin_opaque_token::AdminOpaqueToken::new)?;
    crate::hash_opaque_token::hash_opaque_token(&combined_token)
}
