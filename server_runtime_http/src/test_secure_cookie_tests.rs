#[cfg(test)]
mod tests {
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
}
