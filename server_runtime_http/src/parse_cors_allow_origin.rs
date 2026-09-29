pub fn parse_cors_allow_origin(
    http_cors_allow_origin_text_ref: crate::http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef<'_>,
) -> Result<
    crate::http_cors_allow_origin_header_values::HttpCorsAllowOriginHeaderValues,
    crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError,
> {
    let value_text = http_cors_allow_origin_text_ref.get();
    if value_text.len() > crate::cors_allow_origin_max_bytes::CORS_ALLOW_ORIGIN_MAX_BYTES {
        return Err(crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError::TooLong);
    }
    let capacity = value_text
        .chars()
        .filter(|character| {
            *character == crate::cors_allow_origin_split_ch::CORS_ALLOW_ORIGIN_SPLIT_CH
        })
        .count()
        .saturating_add(constants_usize::ONE);
    if capacity > crate::cors_allow_origin_max_items::CORS_ALLOW_ORIGIN_MAX_ITEMS {
        return Err(crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError::TooManyItems);
    }
    if value_text.trim().is_empty() {
        return Ok(
            crate::http_cors_allow_origin_header_values::HttpCorsAllowOriginHeaderValues::from(
                Vec::new(),
            ),
        );
    }
    let parsed = value_text
        .split(crate::cors_allow_origin_split_ch::CORS_ALLOW_ORIGIN_SPLIT_CH)
        .map(str::trim)
        .map(|origin| {
            let allowed_origin = crate::allowed_origin::AllowedOrigin::try_from(origin.to_owned())
                .map_err(crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError::from)?;
            let default_port = if allowed_origin.scheme().get().eq_ignore_ascii_case(constants_str::HTTP) {
                80u16
            } else {
                443u16
            };
            let configured_port = allowed_origin
                .authority()
                .get()
                .rsplit_once(':')
                .and_then(|(_host, port)| port.parse::<u16>().ok());
            let normalized = origin.to_ascii_lowercase();
            let normalized_origin = if configured_port == Some(default_port) {
                normalized
                    .rsplit_once(':')
                    .map_or(normalized.as_str(), |(without_port, _port)| without_port)
            } else {
                normalized.as_str()
            };
            http::HeaderValue::try_from(normalized_origin)
                .map_err(crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError::from)
        })
        .collect::<Result<Vec<http::HeaderValue>, crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError>>()?;
    Ok(crate::http_cors_allow_origin_header_values::HttpCorsAllowOriginHeaderValues::from(parsed))
}
