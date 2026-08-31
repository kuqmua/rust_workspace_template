#[cfg(test)]
mod tests {
    #[test]
    fn test_parser_trims_valid_origins() {
        let parsed = Vec::<http::HeaderValue>::from(
            crate::parse_cors_allow_origin::parse_cors_allow_origin(
                crate::http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef::from(
                    constants_str::VALUE_BCE3AE6B,
                ),
            )
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
    fn test_parser_preserves_empty_configuration_behavior() {
        let parsed = Vec::<http::HeaderValue>::from(
            crate::parse_cors_allow_origin::parse_cors_allow_origin(
                crate::http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef::from(
                    constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
            )
            .expect("3b681d57 parser_preserves_empty_configuration_behavior invariant must hold"),
        );
        assert!(parsed.is_empty());
    }
    #[test]
    fn test_parser_rejects_invalid_wildcard_and_opaque_origins() {
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
                crate::parse_cors_allow_origin::parse_cors_allow_origin(crate::http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef::from(value)),
                Err(crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError::InvalidOrigin)
            ))
        );
    }
    #[test]
    fn test_parser_rejects_too_many_origins() {
        let value = std::iter::repeat_n(
            constants_str::VALUE_38612C96,
            crate::cors_allow_origin_max_items::CORS_ALLOW_ORIGIN_MAX_ITEMS + 1,
        )
        .collect::<Vec<_>>()
        .join(constants_str::TEXT_ALT_7);
        assert!(matches!(
            crate::parse_cors_allow_origin::parse_cors_allow_origin(
                crate::http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef::from(value.as_str(),)
            ),
            Err(crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError::TooManyItems)
        ));
    }
}

// Root-owned module compatibility wrappers.
