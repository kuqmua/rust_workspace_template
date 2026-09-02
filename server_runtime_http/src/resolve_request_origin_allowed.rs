pub fn resolve_request_origin_allowed(
    http_origin_headers_ref: crate::http_origin_headers_ref::HttpOriginHeadersRef<'_>,
    allowed_origins: &crate::allowed_origins::AllowedOrigins,
) -> crate::request_origin_allowed::RequestOriginAllowed {
    let header_map = http_origin_headers_ref.get();
    let allowed = header_map.get(http::header::ORIGIN).map_or_else(
        || {
            header_map
                .get(http::header::REFERER)
                .and_then(|value| value.to_str().ok())
                .is_some_and(|value| {
                    bool::from(
                        crate::request_origin_value_is_allowed::request_origin_value_is_allowed(
                            crate::http_origin_text_ref::HttpOriginTextRef::from(value),
                            crate::allow_origin_suffix::AllowOriginSuffix::from(true),
                            allowed_origins,
                        ),
                    )
                })
        },
        |origin_header_value| {
            origin_header_value.to_str().is_ok_and(|origin_text| {
                bool::from(
                    crate::request_origin_value_is_allowed::request_origin_value_is_allowed(
                        crate::http_origin_text_ref::HttpOriginTextRef::from(origin_text),
                        crate::allow_origin_suffix::AllowOriginSuffix::from(false),
                        allowed_origins,
                    ),
                )
            })
        },
    );
    crate::request_origin_allowed::RequestOriginAllowed::from(allowed)
}
