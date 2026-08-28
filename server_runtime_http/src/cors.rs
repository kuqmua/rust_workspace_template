#[path = "cors_allow_origin_max_bytes.rs"]
mod cors_allow_origin_max_bytes;
#[path = "cors_allow_origin_max_items.rs"]
mod cors_allow_origin_max_items;
#[path = "cors_allow_origin_split_ch.rs"]
mod cors_allow_origin_split_ch;
#[path = "http_cors_allow_origin_header_values.rs"]
mod http_cors_allow_origin_header_values;
#[path = "http_cors_allow_origin_header_values_error.rs"]
mod http_cors_allow_origin_header_values_error;
#[path = "http_cors_allow_origin_text_ref.rs"]
mod http_cors_allow_origin_text_ref;
#[path = "parse_cors_allow_origin.rs"]
mod parse_cors_allow_origin;

use cors_allow_origin_max_bytes::CORS_ALLOW_ORIGIN_MAX_BYTES;
use cors_allow_origin_max_items::CORS_ALLOW_ORIGIN_MAX_ITEMS;
use cors_allow_origin_split_ch::CORS_ALLOW_ORIGIN_SPLIT_CH;
pub use http_cors_allow_origin_header_values::HttpCorsAllowOriginHeaderValues;
pub use http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError;
pub use http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef;
pub use parse_cors_allow_origin::parse_cors_allow_origin;
#[cfg(test)]
mod tests {
    #[test]
    fn parser_trims_valid_origins() {
        let parsed = Vec::<http::HeaderValue>::from(
            super::parse_cors_allow_origin(super::HttpCorsAllowOriginTextRef::from(
                constants_str::VALUE_BCE3AE6B,
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
                constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
            ))
            .expect("3b681d57 parser_preserves_empty_configuration_behavior invariant must hold"),
        );
        assert!(parsed.is_empty());
    }
    #[test]
    fn parser_rejects_invalid_wildcard_and_opaque_origins() {
        assert!(
            [
                constants_str::HTTPS_A_EXAMPLE_BAD_NEWLINE_VALUE_HTTPS_B_EXAMPLE,
                constants_str::ASTERISK,
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
        let value = std::iter::repeat_n(
            constants_str::VALUE_38612C96,
            super::CORS_ALLOW_ORIGIN_MAX_ITEMS + 1,
        )
        .collect::<Vec<_>>()
        .join(constants_str::TEXT_ALT_7);
        assert!(matches!(
            super::parse_cors_allow_origin(
                super::HttpCorsAllowOriginTextRef::from(value.as_str(),)
            ),
            Err(super::HttpCorsAllowOriginHeaderValuesError::TooManyItems)
        ));
    }
}
