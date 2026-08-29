pub(super) fn request_origin_value_is_allowed(
    value: crate::http_origin_text_ref::HttpOriginTextRef<'_>,
    allow_suffix: crate::allow_origin_suffix::AllowOriginSuffix,
    allowed_origins: &crate::allowed_origins::AllowedOrigins,
) -> crate::request_origin_allowed::RequestOriginAllowed {
    let Some(parsed) = (|| {
        let (scheme, remainder) = value
            .0
            .trim()
            .split_once(constants_str::catalog::TEXT_ALT_10)?;
        if !scheme.eq_ignore_ascii_case(constants_str::catalog::HTTP)
            && !scheme.eq_ignore_ascii_case(constants_str::catalog::HTTPS)
        {
            return None;
        }
        let authority_end = remainder.find(['/', '?', '#']).unwrap_or(remainder.len());
        let authority = remainder.get(..authority_end)?;
        if authority.is_empty() || (!allow_suffix.0 && authority_end != remainder.len()) {
            None
        } else {
            Some(crate::parsed_http_origin_ref::ParsedHttpOriginRef {
                authority: crate::http_origin_text_ref::HttpOriginTextRef::from(authority),
                scheme: crate::http_origin_text_ref::HttpOriginTextRef::from(scheme),
            })
        }
    })() else {
        return crate::request_origin_allowed::RequestOriginAllowed::from(false);
    };
    crate::request_origin_allowed::RequestOriginAllowed::from(allowed_origins.0.iter().any(
        |allowed_origin| {
            allowed_origin
                .scheme
                .0
                .eq_ignore_ascii_case(parsed.scheme.0)
                && allowed_origin
                    .authority
                    .0
                    .eq_ignore_ascii_case(parsed.authority.0)
        },
    ))
}
