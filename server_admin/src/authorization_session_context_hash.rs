pub(crate) fn authorization_session_context_hash(
    headers: crate::HttpAdminHeaderMapRef<'_>,
    peer: crate::AdminPeerAddr,
) -> Result<crate::AdminTokenHash, crate::AdminSecretTextError> {
    let mut context = String::with_capacity(352usize);
    context.push_str(constants_str::CLIENT_ADDRESS);
    let client_address = peer.0.as_ref().ip().to_string();
    context.extend(client_address.chars().take(256usize));
    context.push_str(constants_str::USER_AGENT);
    let user_agent = headers
        .get()
        .get(http::header::USER_AGENT)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|candidate| {
            !candidate.is_empty() && candidate.len() <= constants_usize::VALUE_8_192
        });
    match user_agent {
        Some(normalized_user_agent) => {
            context.extend(normalized_user_agent.chars().take(256usize));
        }
        None => context.push_str(constants_str::UNKNOWN_USER_AGENT),
    }
    let token = crate::SecrecyAdminString::try_from(context).map(crate::AdminOpaqueToken::new)?;
    crate::hash_opaque_token::hash_opaque_token(&token)
}
