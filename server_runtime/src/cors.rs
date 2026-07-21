const CORS_ALLOW_ORIGIN_SPLIT_CH: char = ',';
const CORS_ALLOW_ORIGIN_MAX_BYTES: usize = 65_536usize;
const CORS_ALLOW_ORIGIN_MAX_ITEMS: usize = 128usize;
#[derive(Clone, Copy, Debug)]
pub struct HttpCorsAllowOriginTextRef<'text_lt>(&'text_lt str);
impl<'text_lt> From<&'text_lt str> for HttpCorsAllowOriginTextRef<'text_lt> {
    fn from(value: &'text_lt str) -> Self {
        Self(value)
    }
}
#[derive(Debug)]
pub struct HttpCorsAllowOriginHeaderValues(Vec<http::HeaderValue>);
impl From<Vec<http::HeaderValue>> for HttpCorsAllowOriginHeaderValues {
    fn from(value: Vec<http::HeaderValue>) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum HttpCorsAllowOriginHeaderValuesError {
    #[error("CORS allow-origin configuration exceeds its maximum byte length")]
    TooLong,
    #[error("CORS allow-origin configuration contains too many entries")]
    TooManyItems,
}
impl From<HttpCorsAllowOriginHeaderValues> for Vec<http::HeaderValue> {
    fn from(value: HttpCorsAllowOriginHeaderValues) -> Self {
        value.0
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
    let mut parsed = Vec::with_capacity(capacity);
    parsed.extend(
        value
            .0
            .split(CORS_ALLOW_ORIGIN_SPLIT_CH)
            .filter_map(|part| part.trim().parse::<http::HeaderValue>().ok()),
    );
    Ok(HttpCorsAllowOriginHeaderValues::from(parsed))
}
#[cfg(test)]
mod tests {
    #[test]
    fn parser_trims_valid_origins_and_skips_invalid_values() {
        let parsed = Vec::<http::HeaderValue>::from(
            super::parse_cors_allow_origin(super::HttpCorsAllowOriginTextRef::from(
                str_constants::HTTPS_A_EXAMPLE_BAD_NEWLINE_VALUE_HTTPS_B_EXAMPLE,
            ))
            .expect("d8a0e140"),
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
            .expect("3b681d57"),
        );
        assert_eq!(parsed, vec![http::HeaderValue::from_static("")]);
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
