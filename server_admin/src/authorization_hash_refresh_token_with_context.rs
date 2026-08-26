pub(super) fn authorization_hash_refresh_token_with_context(
    token: &super::super::AdminOpaqueToken,
    context_hash: &super::super::AdminTokenHash,
) -> Result<super::super::AdminTokenHash, super::super::AdminSecretTextError> {
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
    let combined_token = super::super::SecrecyAdminString::try_from(token_with_context)
        .map(super::super::AdminOpaqueToken::new)?;
    super::super::hash_opaque_token::hash_opaque_token(&combined_token)
}
