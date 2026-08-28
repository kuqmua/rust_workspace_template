pub(super) fn request_origin_value_is_allowed(
    value: super::HttpOriginTextRef<'_>,
    allow_suffix: super::AllowOriginSuffix,
    allowed_origins: &super::AllowedOrigins,
) -> super::RequestOriginAllowed {
    let Some(parsed) = (|| {
        let (scheme, remainder) = value.0.trim().split_once(constants_str::TEXT_ALT_10)?;
        if !scheme.eq_ignore_ascii_case(constants_str::HTTP)
            && !scheme.eq_ignore_ascii_case(constants_str::HTTPS)
        {
            return None;
        }
        let authority_end = remainder.find(['/', '?', '#']).unwrap_or(remainder.len());
        let authority = remainder.get(..authority_end)?;
        if authority.is_empty() || (!allow_suffix.0 && authority_end != remainder.len()) {
            None
        } else {
            Some(super::ParsedHttpOriginRef {
                authority: super::HttpOriginTextRef::from(authority),
                scheme: super::HttpOriginTextRef::from(scheme),
            })
        }
    })() else {
        return super::RequestOriginAllowed::from(false);
    };
    super::RequestOriginAllowed::from(allowed_origins.0.iter().any(|allowed_origin| {
        allowed_origin
            .scheme
            .0
            .eq_ignore_ascii_case(parsed.scheme.0)
            && allowed_origin
                .authority
                .0
                .eq_ignore_ascii_case(parsed.authority.0)
    }))
}
