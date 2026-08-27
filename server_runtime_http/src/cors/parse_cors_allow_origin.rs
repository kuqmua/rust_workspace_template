pub fn parse_cors_allow_origin(
    value: super::HttpCorsAllowOriginTextRef<'_>,
) -> Result<super::HttpCorsAllowOriginHeaderValues, super::HttpCorsAllowOriginHeaderValuesError> {
    if value.0.len() > super::CORS_ALLOW_ORIGIN_MAX_BYTES {
        return Err(super::HttpCorsAllowOriginHeaderValuesError::TooLong);
    }
    let capacity = value
        .0
        .chars()
        .filter(|character| *character == super::CORS_ALLOW_ORIGIN_SPLIT_CH)
        .count()
        .saturating_add(constants_usize::ONE);
    if capacity > super::CORS_ALLOW_ORIGIN_MAX_ITEMS {
        return Err(super::HttpCorsAllowOriginHeaderValuesError::TooManyItems);
    }
    if value.0.trim().is_empty() {
        return Ok(super::HttpCorsAllowOriginHeaderValues::from(Vec::new()));
    }
    let parsed = value
        .0
        .split(super::CORS_ALLOW_ORIGIN_SPLIT_CH)
        .map(str::trim)
        .map(|origin| {
            drop(
                super::super::AllowedOrigin::try_from(origin.to_owned())
                    .map_err(super::HttpCorsAllowOriginHeaderValuesError::from)?,
            );
            http::HeaderValue::try_from(origin)
                .map_err(super::HttpCorsAllowOriginHeaderValuesError::from)
        })
        .collect::<Result<Vec<http::HeaderValue>, super::HttpCorsAllowOriginHeaderValuesError>>()?;
    Ok(super::HttpCorsAllowOriginHeaderValues::from(parsed))
}
