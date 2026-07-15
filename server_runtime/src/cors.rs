const CORS_ALLOW_ORIGIN_SPLIT_CH: char = ',';
#[derive(Clone, Copy, Debug)]
pub struct HttpCorsAllowOriginTextRef<'text_lt>(&'text_lt str);
impl<'text_lt> From<&'text_lt str> for HttpCorsAllowOriginTextRef<'text_lt> {
    fn from(value: &'text_lt str) -> Self {
        Self(value)
    }
}
#[derive(Debug)]
pub struct HttpCorsAllowOriginHeaderValues(Vec<http::HeaderValue>);
impl From<HttpCorsAllowOriginHeaderValues> for Vec<http::HeaderValue> {
    fn from(value: HttpCorsAllowOriginHeaderValues) -> Self {
        value.0
    }
}
#[must_use]
pub fn parse_cors_allow_origin(
    value: HttpCorsAllowOriginTextRef<'_>,
) -> HttpCorsAllowOriginHeaderValues {
    let capacity = value
        .0
        .chars()
        .filter(|character| *character == CORS_ALLOW_ORIGIN_SPLIT_CH)
        .count()
        .saturating_add(1usize);
    let mut parsed = Vec::with_capacity(capacity);
    parsed.extend(
        value
            .0
            .split(CORS_ALLOW_ORIGIN_SPLIT_CH)
            .filter_map(|part| part.trim().parse::<http::HeaderValue>().ok()),
    );
    HttpCorsAllowOriginHeaderValues(parsed)
}
#[cfg(test)]
mod tests {
    #[test]
    fn parser_trims_valid_origins_and_skips_invalid_values() {
        let parsed = Vec::<http::HeaderValue>::from(super::parse_cors_allow_origin(
            super::HttpCorsAllowOriginTextRef::from(
                str_constants::HTTPS_A_EXAMPLE_BAD_NEWLINE_VALUE_HTTPS_B_EXAMPLE,
            ),
        ));
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
        let parsed = Vec::<http::HeaderValue>::from(super::parse_cors_allow_origin(
            super::HttpCorsAllowOriginTextRef::from(str_constants::pg_crud::EMPTY_SQL_SUFFIX),
        ));
        assert_eq!(parsed, vec![http::HeaderValue::from_static("")]);
    }
}
