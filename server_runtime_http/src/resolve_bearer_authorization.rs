#[must_use]
pub fn resolve_bearer_authorization(
    header: super::HttpAuthorizationHeaderTextRef<'_>,
) -> super::BearerAuthorizationResolution<'_> {
    let Some(value) = header.0 else {
        return super::BearerAuthorizationResolution::Missing;
    };
    if value.len() > constants_usize::VALUE_4_096 {
        return super::BearerAuthorizationResolution::Invalid;
    }
    let Some((scheme, token)) = value.split_once(' ') else {
        return super::BearerAuthorizationResolution::Invalid;
    };
    if !scheme.eq_ignore_ascii_case(constants_str::BEARER)
        || token.is_empty()
        || token.contains(char::is_whitespace)
    {
        super::BearerAuthorizationResolution::Invalid
    } else {
        super::BearerAuthorizationResolution::Resolved(super::HttpBearerTokenRef::from(token))
    }
}
