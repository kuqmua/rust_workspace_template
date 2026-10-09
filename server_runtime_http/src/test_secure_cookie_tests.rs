#[cfg(test)]
mod tests {
    #[test]
    fn test_cookie_wrappers_ascii_rules_and_value_debug_redaction() {
        assert!((0u8..=127u8).all(|byte| {
            let text = char::from(byte).to_string();
            let name = crate::http_cookie_name::HttpCookieName::try_from(text.clone());
            let value = crate::http_cookie_value::HttpCookieValue::try_from(text.clone());
            let name_allowed = byte.is_ascii_alphanumeric()
                || matches!(
                    byte,
                    b'!' | b'#'
                        | b'$'
                        | b'%'
                        | b'&'
                        | b'\''
                        | b'*'
                        | b'+'
                        | b'-'
                        | b'.'
                        | b'^'
                        | b'_'
                        | b'`'
                        | b'|'
                        | b'~'
                );
            let value_allowed =
                (33u8..=126u8).contains(&byte) && !matches!(byte, b'"' | b',' | b';' | b'\\');
            let name_matches = if name_allowed {
                name.is_ok_and(|http_cookie_name| http_cookie_name.as_str() == text)
            } else {
                name == Err(crate::http_secure_cookie_error::HttpSecureCookieError::InvalidName)
            };
            let value_matches = if value_allowed {
                value.is_ok_and(|http_cookie_value| {
                    http_cookie_value.as_str() == text
                        && format!("{http_cookie_value:?}") == constants_str::REDACTED_ALT_3
                })
            } else {
                value == Err(crate::http_secure_cookie_error::HttpSecureCookieError::InvalidValue)
            };
            name_matches && value_matches
        }));
    }

    #[test]
    fn test_cookie_wrappers_reject_non_ascii_characters() {
        assert!(
            (128u8..=255u8)
                .map(char::from)
                .chain(['\u{800}', '\u{10000}', '\u{10ffff}'])
                .all(|character| {
                    let text = character.to_string();
                    crate::http_cookie_name::HttpCookieName::try_from(text.clone())
                    == Err(crate::http_secure_cookie_error::HttpSecureCookieError::InvalidName)
                    && crate::http_cookie_value::HttpCookieValue::try_from(text)
                        == Err(crate::http_secure_cookie_error::HttpSecureCookieError::InvalidValue)
                })
        );
    }

    #[test]
    fn test_cookie_wrapper_empty_and_exact_size_limits() {
        assert!([0usize, 1usize, 8191usize, 8192usize, 8193usize].into_iter().all(|length| {
            let text = constants_str::X.repeat(length);
            let name = crate::http_cookie_name::HttpCookieName::try_from(text.clone());
            let value = crate::http_cookie_value::HttpCookieValue::try_from(text.clone());
            let name_matches = if (1usize..=8192usize).contains(&length) {
                name.is_ok_and(|http_cookie_name| http_cookie_name.as_str() == text)
            } else {
                name == Err(crate::http_secure_cookie_error::HttpSecureCookieError::InvalidName)
            };
            let value_matches = if length <= 8192usize {
                value.is_ok_and(|http_cookie_value| http_cookie_value.as_str() == text)
            } else {
                value == Err(crate::http_secure_cookie_error::HttpSecureCookieError::InvalidValue)
            };
            name_matches && value_matches
        }));
    }

    #[test]
    fn test_builder_sets_security_attributes_and_rejects_injection() {
        let name = crate::http_cookie_name::HttpCookieName::try_from(String::from(
            constants_str::TEST_COOKIE_NAME,
        ))
        .expect(constants_str::DIAGNOSTIC_977F74F0);
        let value = crate::http_cookie_value::HttpCookieValue::try_from(String::from(
            constants_str::TEST_COOKIE_VALUE,
        ))
        .expect(constants_str::DIAGNOSTIC_38FC5531);
        let header = crate::build_secure_strict_cookie::build_secure_strict_cookie(
            &name,
            &value,
            60u64.into(),
            crate::http_cookie_access::HttpCookieAccess::HttpOnly,
            crate::http_cookie_secure::HttpCookieSecure::Enabled,
        )
        .expect(constants_str::DIAGNOSTIC_0B4600B3);
        let header_value = http::HeaderValue::from(header);
        let text = header_value
            .to_str()
            .expect(constants_str::DIAGNOSTIC_3176FB72);
        assert!(text.contains(constants_str::HTTPONLY));
        assert!(text.contains(constants_str::SECURE));
        assert_eq!(
            crate::http_cookie_value::HttpCookieValue::try_from(String::from(
                constants_str::TEST_COOKIE_INJECTION
            )),
            Err(crate::http_secure_cookie_error::HttpSecureCookieError::InvalidValue),
        );
        assert_eq!(
            crate::http_cookie_name::HttpCookieName::try_from(String::from(
                constants_str::VALUE_A463C738
            )),
            Err(crate::http_secure_cookie_error::HttpSecureCookieError::InvalidName),
        );
        assert_eq!(
            crate::http_cookie_name::HttpCookieName::try_from(String::from(
                constants_str::VALUE_D071C324
            )),
            Err(crate::http_secure_cookie_error::HttpSecureCookieError::InvalidName),
        );
    }

    #[test]
    fn test_builder_preserves_unsigned_maximum_age_range() {
        let name = crate::http_cookie_name::HttpCookieName::try_from(String::from(
            constants_str::TEST_COOKIE_NAME,
        ))
        .expect(constants_str::DIAGNOSTIC_3DDE3FF2);
        let value = crate::http_cookie_value::HttpCookieValue::try_from(String::from(
            constants_str::TEST_COOKIE_VALUE,
        ))
        .expect(constants_str::DIAGNOSTIC_7B47E5B5);
        let header = crate::build_secure_strict_cookie::build_secure_strict_cookie(
            &name,
            &value,
            u64::MAX.into(),
            crate::http_cookie_access::HttpCookieAccess::ScriptReadable,
            crate::http_cookie_secure::HttpCookieSecure::Disabled,
        )
        .expect(constants_str::DIAGNOSTIC_0A722D46);
        assert!(
            http::HeaderValue::from(header)
                .to_str()
                .expect(constants_str::DIAGNOSTIC_B1DDE58F)
                .contains(u64::MAX.to_string().as_str())
        );
    }

    #[test]
    fn test_maximum_cookie_name_and_value_round_trip_through_resolver() {
        let name = crate::http_cookie_name::HttpCookieName::try_from(
            constants_str::A_ALT.repeat(constants_usize::VALUE_8_192),
        )
        .expect(constants_str::DIAGNOSTIC_9AD2B231);
        let value = crate::http_cookie_value::HttpCookieValue::try_from(
            constants_str::A_ALT.repeat(constants_usize::VALUE_8_192),
        )
        .expect(constants_str::DIAGNOSTIC_B6D60E61);
        let _set_cookie = crate::build_secure_strict_cookie::build_secure_strict_cookie(
            &name,
            &value,
            60u64.into(),
            crate::http_cookie_access::HttpCookieAccess::HttpOnly,
            crate::http_cookie_secure::HttpCookieSecure::Enabled,
        )
        .expect(constants_str::DIAGNOSTIC_FFFC783E);
        let mut cookie_text = String::with_capacity(
            name.as_str()
                .len()
                .saturating_add(constants_usize::ONE)
                .saturating_add(value.as_str().len()),
        );
        cookie_text.push_str(name.as_str());
        cookie_text.push('=');
        cookie_text.push_str(value.as_str());
        let mut headers = http::HeaderMap::new();
        let _previous_cookie_header = headers.insert(
            http::header::COOKIE,
            http::HeaderValue::try_from(cookie_text.as_str())
                .expect(constants_str::DIAGNOSTIC_42E776DD),
        );
        assert_eq!(
            crate::resolve_unique_cookie::resolve_unique_cookie(
                crate::http_cookie_headers_ref::HttpCookieHeadersRef::from(&headers),
                crate::http_cookie_name_ref::HttpCookieNameRef::from(name.as_str()),
            ),
            crate::cookie_resolution::CookieResolution::Resolved(
                crate::http_cookie_value_ref::HttpCookieValueRef::from(value.as_str())
            )
        );
        cookie_text.push('a');
        let _replaced_cookie_header = headers.insert(
            http::header::COOKIE,
            http::HeaderValue::try_from(cookie_text).expect(constants_str::DIAGNOSTIC_84F82554),
        );
        assert_eq!(
            crate::resolve_unique_cookie::resolve_unique_cookie(
                crate::http_cookie_headers_ref::HttpCookieHeadersRef::from(&headers),
                crate::http_cookie_name_ref::HttpCookieNameRef::from(name.as_str()),
            ),
            crate::cookie_resolution::CookieResolution::Invalid
        );
    }

    #[test]
    fn test_cookie_builder_preserves_security_flag_matrix_and_age_conversion_boundaries() {
        let name_result = crate::http_cookie_name::HttpCookieName::try_from(
            constants_str::TEST_COOKIE_NAME.to_owned(),
        );
        let value_result = crate::http_cookie_value::HttpCookieValue::try_from(
            constants_str::TEST_COOKIE_VALUE.to_owned(),
        );
        assert!(name_result.is_ok() && value_result.is_ok());
        let (Ok(name), Ok(value)) = (name_result, value_result) else {
            return;
        };
        assert!(
            [
                0u64,
                60u64,
                i64::MAX.unsigned_abs(),
                i64::MAX.unsigned_abs() + 1u64,
                u64::MAX
            ]
            .into_iter()
            .all(|age| {
                [
                    (crate::http_cookie_access::HttpCookieAccess::HttpOnly, true),
                    (
                        crate::http_cookie_access::HttpCookieAccess::ScriptReadable,
                        false,
                    ),
                ]
                .into_iter()
                .all(|(access, http_only)| {
                    [
                        (crate::http_cookie_secure::HttpCookieSecure::Disabled, false),
                        (crate::http_cookie_secure::HttpCookieSecure::Enabled, true),
                    ]
                    .into_iter()
                    .all(|(security, secure)| {
                        let expected = [
                            format!(
                                "{}={}",
                                constants_str::TEST_COOKIE_NAME,
                                constants_str::TEST_COOKIE_VALUE
                            ),
                            format!("{}=/", stringify!(Path)),
                            format!("{}-{}={age}", stringify!(Max), stringify!(Age)),
                            format!("{}={}", stringify!(SameSite), stringify!(Strict)),
                        ];
                        crate::build_secure_strict_cookie::build_secure_strict_cookie(
                            &name,
                            &value,
                            crate::std_cookie_max_age_seconds::StdCookieMaxAgeSeconds::from(age),
                            access,
                            security,
                        )
                        .is_ok_and(|header| {
                            let header_value = http::HeaderValue::from(header);
                            header_value.to_str().is_ok_and(|text| {
                                expected.iter().all(|attribute| {
                                    text.split(';')
                                        .map(str::trim)
                                        .any(|observed| observed == attribute)
                                }) && text
                                    .split(';')
                                    .map(str::trim)
                                    .any(|attribute| attribute == stringify!(HttpOnly))
                                    == http_only
                                    && text
                                        .split(';')
                                        .map(str::trim)
                                        .any(|attribute| attribute == stringify!(Secure))
                                        == secure
                                    && text.split(';').count()
                                        == expected.len()
                                            + usize::from(http_only)
                                            + usize::from(secure)
                            })
                        })
                    })
                })
            })
        );
    }
}
