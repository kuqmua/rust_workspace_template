#[must_use]
pub fn resolve_bearer_authorization(
    header: crate::http_authorization_header_text_ref::HttpAuthorizationHeaderTextRef<'_>,
) -> crate::bearer_authorization_resolution::BearerAuthorizationResolution<'_> {
    let Some(value) = header.0 else {
        return crate::bearer_authorization_resolution::BearerAuthorizationResolution::Missing;
    };
    if value.len() > constants_usize::VALUE_4_096 {
        return crate::bearer_authorization_resolution::BearerAuthorizationResolution::Invalid;
    }
    let Some((scheme, token)) = value.split_once(' ') else {
        return crate::bearer_authorization_resolution::BearerAuthorizationResolution::Invalid;
    };
    if !scheme.eq_ignore_ascii_case(constants_str::BEARER)
        || token.is_empty()
        || token.contains(char::is_whitespace)
    {
        crate::bearer_authorization_resolution::BearerAuthorizationResolution::Invalid
    } else {
        crate::bearer_authorization_resolution::BearerAuthorizationResolution::Resolved(
            crate::http_bearer_token_ref::HttpBearerTokenRef::from(token),
        )
    }
}
