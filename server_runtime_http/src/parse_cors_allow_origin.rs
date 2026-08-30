pub fn parse_cors_allow_origin(
    value: crate::http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef<'_>,
) -> Result<
    crate::http_cors_allow_origin_header_values::HttpCorsAllowOriginHeaderValues,
    crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError,
> {
    let value_text = value.get();
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
            drop(
                crate::allowed_origin::AllowedOrigin::try_from(origin.to_owned())
                    .map_err(crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError::from)?,
            );
            http::HeaderValue::try_from(origin)
                .map_err(crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError::from)
        })
        .collect::<Result<Vec<http::HeaderValue>, crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError>>()?;
    Ok(crate::http_cors_allow_origin_header_values::HttpCorsAllowOriginHeaderValues::from(parsed))
}
