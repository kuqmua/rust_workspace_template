#[cfg(test)]
mod tests {
    #[test]
    fn test_bearer_authorization_distinguishes_missing_headers_and_invalid_schemes() {
        let cases = [
            None,
            Some(String::new()),
            Some(constants_str::BEARER.to_owned()),
            Some(format!("{}{}", constants_str::BEARER, constants_str::SPACE)),
            Some(format!(
                "{}{}{}",
                constants_str::X,
                constants_str::SPACE,
                constants_str::SECRET,
            )),
            Some(format!(
                "{}{}{}{}",
                constants_str::BEARER,
                constants_str::X,
                constants_str::SPACE,
                constants_str::SECRET,
            )),
            Some(format!(
                "{}{}{}{}",
                constants_str::SPACE,
                constants_str::BEARER,
                constants_str::SPACE,
                constants_str::SECRET,
            )),
            Some(
                constants_str::BEARER
                    .chars()
                    .chain(std::iter::once('\t'))
                    .chain(constants_str::SECRET.chars())
                    .collect::<String>(),
            ),
        ];
        assert!(cases.into_iter().all(|header| {
            let expected = if header.is_some() {
                crate::bearer_authorization_resolution::BearerAuthorizationResolution::Invalid
            } else {
                crate::bearer_authorization_resolution::BearerAuthorizationResolution::Missing
            };
            crate::resolve_bearer_authorization::resolve_bearer_authorization(
                crate::http_authorization_header_text_ref::HttpAuthorizationHeaderTextRef::from(
                    header.as_deref(),
                ),
            ) == expected
        }));
    }

    #[test]
    fn test_bearer_header_size_limit_includes_scheme_and_spacing() {
        assert!([1usize, 2usize, 3usize].into_iter().all(|spaces| {
            [4095usize, 4096usize, 4097usize].into_iter().all(|total_length| {
                let mut header = constants_str::BEARER.to_ascii_lowercase();
                header.push_str(&' '.to_string().repeat(spaces));
                let token = constants_str::X.repeat(total_length.saturating_sub(header.len()));
                header.push_str(&token);
                let actual = crate::resolve_bearer_authorization::resolve_bearer_authorization(
                    crate::http_authorization_header_text_ref::HttpAuthorizationHeaderTextRef::from(Some(header.as_str())),
                );
                if total_length == 4097usize {
                    actual == crate::bearer_authorization_resolution::BearerAuthorizationResolution::Invalid
                } else {
                    actual == crate::bearer_authorization_resolution::BearerAuthorizationResolution::Resolved(
                        crate::http_bearer_token_ref::HttpBearerTokenRef::from(token.as_str()),
                    )
                }
            })
        }));
    }

    #[test]
    fn test_bearer_token_validates_every_ascii_suffix_and_preserves_padding() {
        assert!((0u8..=127u8).all(|byte| {
            let mut token = constants_str::X.to_owned();
            token.push(char::from(byte));
            let mut header = constants_str::BEARER.to_owned();
            header.push(' ');
            header.push_str(&token);
            let actual = crate::resolve_bearer_authorization::resolve_bearer_authorization(
                crate::http_authorization_header_text_ref::HttpAuthorizationHeaderTextRef::from(Some(header.as_str())),
            );
            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~' | b'+' | b'/' | b'=') {
                actual == crate::bearer_authorization_resolution::BearerAuthorizationResolution::Resolved(
                    crate::http_bearer_token_ref::HttpBearerTokenRef::from(token.as_str()),
                )
            } else {
                actual == crate::bearer_authorization_resolution::BearerAuthorizationResolution::Invalid
            }
        }));
    }

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
    fn test_bearer_authorization_accepts_spaces_and_trailing_padding() {
        assert!([
            (constants_str::TEST_BEARER_AUTHORIZATION_MULTIPLE_SPACES, constants_str::SECRET),
            (constants_str::TEST_BEARER_AUTHORIZATION_PADDED, constants_str::TEST_BEARER_TOKEN_PADDED),
        ].into_iter().all(|(header, expected)| matches!(
            crate::resolve_bearer_authorization::resolve_bearer_authorization(
                crate::http_authorization_header_text_ref::HttpAuthorizationHeaderTextRef::from(Some(header)),
            ),
            crate::bearer_authorization_resolution::BearerAuthorizationResolution::Resolved(token)
                if token == crate::http_bearer_token_ref::HttpBearerTokenRef::from(expected)
        )));
        assert!(['-', '.', '_', '~', '+', '/'].into_iter().all(|character| {
            let mut header = constants_str::TEST_BEARER_AUTHORIZATION.to_owned();
            header.push(character);
            matches!(
                crate::resolve_bearer_authorization::resolve_bearer_authorization(
                    crate::http_authorization_header_text_ref::HttpAuthorizationHeaderTextRef::from(
                        Some(header.as_str())
                    ),
                ),
                crate::bearer_authorization_resolution::BearerAuthorizationResolution::Resolved(_)
            )
        }));
    }

    #[test]
    fn test_bearer_authorization_rejects_invalid_characters_and_padding() {
        assert!(
            [
                constants_str::TEST_BEARER_AUTHORIZATION_INTERIOR_PADDING,
                constants_str::TEST_BEARER_AUTHORIZATION_ONLY_PADDING,
            ]
            .into_iter()
            .all(|header| matches!(
                crate::resolve_bearer_authorization::resolve_bearer_authorization(
                    crate::http_authorization_header_text_ref::HttpAuthorizationHeaderTextRef::from(
                        Some(header)
                    ),
                ),
                crate::bearer_authorization_resolution::BearerAuthorizationResolution::Invalid
            ))
        );
        assert!(
            [':', '\0', '\u{7f}', '\u{e9}']
                .into_iter()
                .all(|character| {
                    let mut header = constants_str::TEST_BEARER_AUTHORIZATION.to_owned();
                    header.push(character);
                    matches!(
                crate::resolve_bearer_authorization::resolve_bearer_authorization(
                    crate::http_authorization_header_text_ref::HttpAuthorizationHeaderTextRef::from(
                        Some(header.as_str())
                    ),
                ),
                crate::bearer_authorization_resolution::BearerAuthorizationResolution::Invalid
            )
                })
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
