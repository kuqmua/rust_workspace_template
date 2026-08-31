#[cfg(test)]
mod tests {
    #[test]
    fn test_bearer_authorization_requires_exact_scheme_and_token() {
        assert!(matches!(
            crate::resolve_bearer_authorization::resolve_bearer_authorization(
                crate::http_authorization_header_text_ref::HttpAuthorizationHeaderTextRef::from(
                    Some(constants_str::TEST_BEARER_AUTHORIZATION)
                )
            ),
            crate::bearer_authorization_resolution::BearerAuthorizationResolution::Resolved(_)
        ));
        let secret = constants_str::NEVER_PRINT_THIS_VALUE;
        assert!(
            !format!(
                "{:?}",
                crate::http_bearer_token_ref::HttpBearerTokenRef::from(secret)
            )
            .contains(secret)
        );
        assert!(
            !format!(
                "{:?}",
                crate::http_cookie_value_ref::HttpCookieValueRef::from(secret)
            )
            .contains(secret)
        );
    }
    #[test]
    fn test_duplicate_cookie_is_invalid() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            http::header::COOKIE,
            http::HeaderValue::from_static(constants_str::TEST_DUPLICATE_COOKIE),
        );
        assert_eq!(
            crate::resolve_unique_cookie::resolve_unique_cookie(
                crate::http_cookie_headers_ref::HttpCookieHeadersRef::from(&headers),
                crate::http_cookie_name_ref::HttpCookieNameRef::from(
                    constants_str::TEST_COOKIE_NAME
                )
            ),
            crate::cookie_resolution::CookieResolution::Invalid
        );
    }
    #[test]
    fn test_json_content_type_supports_charset() {
        assert_eq!(
            crate::classify_optional_json_content_type::classify_optional_json_content_type(
                crate::http_content_type_text_ref::HttpContentTypeTextRef::from(Some(
                    constants_str::TEST_JSON_CONTENT_TYPE_WITH_CHARSET
                ))
            ),
            crate::optional_json_content_type::OptionalJsonContentType::ApplicationJson
        );
    }
    #[test]
    fn test_optional_json_rejects_non_json_non_empty_body() {
        assert_eq!(
            crate::resolve_optional_json_content_type_decision::resolve_optional_json_content_type_decision(
                crate::optional_json_body_presence::OptionalJsonBodyPresence::NonEmpty,
                crate::optional_json_content_type::OptionalJsonContentType::NonJson
            ),
            crate::optional_json_content_type_decision::OptionalJsonContentTypeDecision::RejectUnsupportedMediaType
        );
    }
}

// Root-owned module compatibility wrappers.
