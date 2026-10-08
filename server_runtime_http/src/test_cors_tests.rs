#[cfg(test)]
mod tests {
    #[test]
    fn test_cors_configuration_byte_limit_precedes_whitespace_trimming() {
        assert!([65535usize, 65536usize, 65537usize].into_iter().all(|length| {
            [constants_str::PG_CRUD_EMPTY_SQL_SUFFIX, constants_str::HTTP_LOCALHOST].into_iter().all(|origin| {
                let text = format!("{}{origin}", constants_str::SPACE.repeat(length.saturating_sub(origin.len())));
                let result = crate::parse_cors_allow_origin::parse_cors_allow_origin(
                    crate::http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef::from(text.as_str()),
                );
                if length > 65536usize {
                    matches!(result, Err(crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError::TooLong))
                } else {
                    result.is_ok_and(|values| {
                        let headers = Vec::<http::HeaderValue>::from(values);
                        if origin.is_empty() {
                            headers.is_empty()
                        } else {
                            headers == [http::HeaderValue::from_static(constants_str::HTTP_LOCALHOST)]
                        }
                    })
                }
            })
        }));
    }

    #[test]
    fn test_cors_configuration_item_limit_preserves_duplicate_origins() {
        assert!([0usize, 1usize, 127usize, 128usize, 129usize].into_iter().all(|count| {
            let text = vec![constants_str::HTTP_LOCALHOST; count].join(constants_str::TEXT_ALT_7);
            let result = crate::parse_cors_allow_origin::parse_cors_allow_origin(
                crate::http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef::from(text.as_str()),
            );
            if count > 128usize {
                matches!(result, Err(crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError::TooManyItems))
            } else {
                result.is_ok_and(|values| {
                    let headers = Vec::<http::HeaderValue>::from(values);
                    headers.len() == count && headers.iter().all(|header| header.as_bytes() == constants_str::HTTP_LOCALHOST.as_bytes())
                })
            }
        }));
    }

    #[test]
    fn test_parser_trims_valid_origins() {
        let parsed = Vec::<http::HeaderValue>::from(
            crate::parse_cors_allow_origin::parse_cors_allow_origin(
                crate::http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef::from(
                    constants_str::VALUE_BCE3AE6B,
                ),
            )
            .expect(constants_str::DIAGNOSTIC_D8A0E140),
        );
        assert_eq!(
            parsed,
            [
                http::HeaderValue::from_static(constants_str::VALUE_38612C96),
                http::HeaderValue::from_static(constants_str::VALUE_35B22C6C),
            ]
        );
    }
    #[test]
    fn test_parser_normalizes_origin_scheme_and_host_case() {
        let uppercase = constants_str::HTTPS_ADMIN_EXAMPLE_COM.to_ascii_uppercase();
        let parsed = crate::parse_cors_allow_origin::parse_cors_allow_origin(
            crate::http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef::from(
                uppercase.as_str(),
            ),
        )
        .map(Vec::<http::HeaderValue>::from);
        assert_eq!(
            parsed,
            Ok(vec![http::HeaderValue::from_static(
                constants_str::HTTPS_ADMIN_EXAMPLE_COM
            )])
        );
    }
    #[test]
    fn test_parser_omits_default_origin_ports() {
        let cases = [
            (constants_str::HTTP_LOCALHOST, 80u16),
            (constants_str::HTTPS_ADMIN_EXAMPLE_COM, 443u16),
        ];
        assert!(cases.into_iter().all(|(origin, port)| {
            let configured = format!("{origin}:{port}");
            let parsed = crate::parse_cors_allow_origin::parse_cors_allow_origin(
                crate::http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef::from(
                    configured.as_str(),
                ),
            )
            .map(Vec::<http::HeaderValue>::from);
            parsed == Ok(vec![http::HeaderValue::from_static(origin)])
        }));
    }
    #[test]
    fn test_parser_preserves_empty_configuration_behavior() {
        let parsed = Vec::<http::HeaderValue>::from(
            crate::parse_cors_allow_origin::parse_cors_allow_origin(
                crate::http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef::from(
                    constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
            )
            .expect(constants_str::DIAGNOSTIC_3B681D57),
        );
        assert!(parsed.is_empty());
    }
    #[test]
    fn test_parser_rejects_invalid_wildcard_and_opaque_origins() {
        assert!(
            [
                constants_str::HTTPS_A_EXAMPLE_BAD_NEWLINE_VALUE_HTTPS_B_EXAMPLE,
                constants_str::ASTERISK,
                constants_str::JSON_NULL,
                constants_str::VALUE_1FE3969E,
                constants_str::VALUE_1C98CF2D,
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

    #[test]
    fn test_cors_error_precedence_preserves_byte_item_and_origin_validation_order() {
        let resolve = |http_cors_allow_origin_text_ref: crate::http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef<'_>| {
            crate::parse_cors_allow_origin::parse_cors_allow_origin(
                http_cors_allow_origin_text_ref,
            )
        };
        let oversized = ','.to_string().repeat(65_537usize);
        assert!(matches!(resolve(crate::http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef::from(oversized.as_str())), Err(crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError::TooLong)));
        let excessive_items = ','.to_string().repeat(128usize);
        assert!(matches!(resolve(crate::http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef::from(excessive_items.as_str())), Err(crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError::TooManyItems)));
        assert!([
            ','.to_string(),
            format!(",{}", constants_str::HTTP_LOCALHOST),
            format!("{},", constants_str::HTTP_LOCALHOST),
            format!("{},,{}", constants_str::HTTP_LOCALHOST, constants_str::HTTPS_ADMIN_EXAMPLE_COM),
        ].into_iter().all(|text| matches!(resolve(crate::http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef::from(text.as_str())), Err(crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError::InvalidOrigin))));
    }
}
