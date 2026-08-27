#[must_use]
pub fn request_origin_allowed(
    headers: super::super::HttpOriginHeadersRef<'_>,
    allowed_origins: &super::super::AllowedOrigins,
) -> super::RequestOriginAllowed {
    let allowed = headers.0.get(http::header::ORIGIN).map_or_else(
        || {
            headers
                .0
                .get(http::header::REFERER)
                .and_then(|value| value.to_str().ok())
                .is_some_and(|value| {
                    bool::from(super::super::request_origin_value_is_allowed(
                        super::super::HttpOriginTextRef::from(value),
                        super::super::AllowOriginSuffix::from(true),
                        allowed_origins,
                    ))
                })
        },
        |origin_header_value| {
            origin_header_value.to_str().is_ok_and(|origin_text| {
                bool::from(super::super::request_origin_value_is_allowed(
                    super::super::HttpOriginTextRef::from(origin_text),
                    super::super::AllowOriginSuffix::from(false),
                    allowed_origins,
                ))
            })
        },
    );
    super::RequestOriginAllowed::from(allowed)
}
