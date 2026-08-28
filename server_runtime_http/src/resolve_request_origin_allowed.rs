use super::{
    AllowOriginSuffix, AllowedOrigins, HttpOriginHeadersRef, HttpOriginTextRef,
    RequestOriginAllowed, request_origin_value_is_allowed,
};

pub fn resolve_request_origin_allowed(
    headers: HttpOriginHeadersRef<'_>,
    allowed_origins: &AllowedOrigins,
) -> RequestOriginAllowed {
    let allowed = headers.0.get(http::header::ORIGIN).map_or_else(
        || {
            headers
                .0
                .get(http::header::REFERER)
                .and_then(|value| value.to_str().ok())
                .is_some_and(|value| {
                    bool::from(request_origin_value_is_allowed(
                        HttpOriginTextRef::from(value),
                        AllowOriginSuffix::from(true),
                        allowed_origins,
                    ))
                })
        },
        |origin_header_value| {
            origin_header_value.to_str().is_ok_and(|origin_text| {
                bool::from(request_origin_value_is_allowed(
                    HttpOriginTextRef::from(origin_text),
                    AllowOriginSuffix::from(false),
                    allowed_origins,
                ))
            })
        },
    );
    RequestOriginAllowed::from(allowed)
}
