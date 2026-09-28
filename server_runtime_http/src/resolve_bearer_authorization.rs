#[must_use]
pub fn resolve_bearer_authorization(
    http_authorization_header_text_ref: crate::http_authorization_header_text_ref::HttpAuthorizationHeaderTextRef<'_>,
) -> crate::bearer_authorization_resolution::BearerAuthorizationResolution<'_> {
    let Some(value) = http_authorization_header_text_ref.get() else {
        return crate::bearer_authorization_resolution::BearerAuthorizationResolution::Missing;
    };
    if value.len() > constants_usize::VALUE_4_096 {
        return crate::bearer_authorization_resolution::BearerAuthorizationResolution::Invalid;
    }
    let Some((scheme, token)) = value.split_once(' ') else {
        return crate::bearer_authorization_resolution::BearerAuthorizationResolution::Invalid;
    };
    let token_text = token.trim_start_matches(' ');
    let unpadded_token = token_text.trim_end_matches('=');
    if !scheme.eq_ignore_ascii_case(constants_str::BEARER)
        || unpadded_token.is_empty()
        || !unpadded_token.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~' | b'+' | b'/')
        })
    {
        crate::bearer_authorization_resolution::BearerAuthorizationResolution::Invalid
    } else {
        crate::bearer_authorization_resolution::BearerAuthorizationResolution::Resolved(
            crate::http_bearer_token_ref::HttpBearerTokenRef::from(token_text),
        )
    }
}
