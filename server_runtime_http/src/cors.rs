const CORS_ALLOW_ORIGIN_SPLIT_CH: char = ',';
const CORS_ALLOW_ORIGIN_MAX_BYTES: usize = 65_536usize;
const CORS_ALLOW_ORIGIN_MAX_ITEMS: usize = 128usize;
#[derive(optml::Optml, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpCorsAllowOriginTextRef<'text_lt>(&'text_lt str);

#[derive(optml::Optml, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct HttpCorsAllowOriginHeaderValues(Vec<http::HeaderValue>);

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum HttpCorsAllowOriginHeaderValuesError {
    #[error("CORS allow-origin configuration contains an invalid origin")]
    InvalidOrigin,
    #[error("CORS allow-origin configuration exceeds its maximum byte length")]
    TooLong,
    #[error("CORS allow-origin configuration contains too many entries")]
    TooManyItems,
}
impl From<super::AllowedOriginError> for HttpCorsAllowOriginHeaderValuesError {
    fn from(_value: super::AllowedOriginError) -> Self {
        Self::InvalidOrigin
    }
}
impl From<http::header::InvalidHeaderValue> for HttpCorsAllowOriginHeaderValuesError {
    fn from(_value: http::header::InvalidHeaderValue) -> Self {
        Self::InvalidOrigin
    }
}
pub fn parse_cors_allow_origin(
    value: HttpCorsAllowOriginTextRef<'_>,
) -> Result<HttpCorsAllowOriginHeaderValues, HttpCorsAllowOriginHeaderValuesError> {
    if value.0.len() > CORS_ALLOW_ORIGIN_MAX_BYTES {
        return Err(HttpCorsAllowOriginHeaderValuesError::TooLong);
    }
    let capacity = value
        .0
        .chars()
        .filter(|character| *character == CORS_ALLOW_ORIGIN_SPLIT_CH)
        .count()
        .saturating_add(1usize);
    if capacity > CORS_ALLOW_ORIGIN_MAX_ITEMS {
        return Err(HttpCorsAllowOriginHeaderValuesError::TooManyItems);
    }
    if value.0.trim().is_empty() {
        return Ok(HttpCorsAllowOriginHeaderValues::from(Vec::new()));
    }
    let parsed = value
        .0
        .split(CORS_ALLOW_ORIGIN_SPLIT_CH)
        .map(str::trim)
        .map(|origin| {
            drop(
                super::AllowedOrigin::try_from(origin.to_owned())
                    .map_err(HttpCorsAllowOriginHeaderValuesError::from)?,
            );
            http::HeaderValue::try_from(origin).map_err(HttpCorsAllowOriginHeaderValuesError::from)
        })
        .collect::<Result<Vec<http::HeaderValue>, HttpCorsAllowOriginHeaderValuesError>>()?;
    Ok(HttpCorsAllowOriginHeaderValues::from(parsed))
}
#[cfg(test)]
mod tests {
    #[test]
    fn parser_trims_valid_origins() {
        let parsed = Vec::<http::HeaderValue>::from(
            super::parse_cors_allow_origin(super::HttpCorsAllowOriginTextRef::from(
                " https://a.example , https://b.example ",
            ))
            .expect("d8a0e140 parser_trims_valid_origins invariant must hold"),
        );
        assert_eq!(
            parsed,
            vec![
                http::HeaderValue::from_static("https://a.example"),
                http::HeaderValue::from_static("https://b.example"),
            ]
        );
    }
    #[test]
    fn parser_preserves_empty_configuration_behavior() {
        let parsed = Vec::<http::HeaderValue>::from(
            super::parse_cors_allow_origin(super::HttpCorsAllowOriginTextRef::from(
                str_constants::PG_CRUD_EMPTY_SQL_SUFFIX,
            ))
            .expect("3b681d57 parser_preserves_empty_configuration_behavior invariant must hold"),
        );
        assert!(parsed.is_empty());
    }
    #[test]
    fn parser_rejects_invalid_wildcard_and_opaque_origins() {
        assert!(
            [
                str_constants::HTTPS_A_EXAMPLE_BAD_NEWLINE_VALUE_HTTPS_B_EXAMPLE,
                str_constants::ASTERISK,
                "null",
                "https://a.example/path",
                "https://a.example,,https://b.example",
            ]
            .into_iter()
            .all(|value| matches!(
                super::parse_cors_allow_origin(super::HttpCorsAllowOriginTextRef::from(value)),
                Err(super::HttpCorsAllowOriginHeaderValuesError::InvalidOrigin)
            ))
        );
    }
    #[test]
    fn parser_rejects_too_many_origins() {
        let value =
            std::iter::repeat_n("https://a.example", super::CORS_ALLOW_ORIGIN_MAX_ITEMS + 1)
                .collect::<Vec<_>>()
                .join(",");
        assert!(matches!(
            super::parse_cors_allow_origin(
                super::HttpCorsAllowOriginTextRef::from(value.as_str(),)
            ),
            Err(super::HttpCorsAllowOriginHeaderValuesError::TooManyItems)
        ));
    }
}
