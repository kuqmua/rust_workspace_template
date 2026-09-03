#[cfg(test)]
mod tests {
    fn allowed_origins() -> crate::allowed_origins::AllowedOrigins {
        crate::allowed_origins::AllowedOrigins::try_from(vec![String::from(
            constants_str::HTTPS_ADMIN_EXAMPLE_COM,
        )])
        .expect(constants_str::DIAGNOSTIC_782D2BED)
    }

    #[test]
    fn test_allowed_origins_reject_oversized_lists() {
        let values = vec![String::from(constants_str::HTTPS_ADMIN_EXAMPLE_COM); 129usize];
        assert_eq!(
            crate::allowed_origins::AllowedOrigins::try_from(values),
            Err(crate::allowed_origins_error::AllowedOriginsError::Invalid)
        );
    }

    #[test]
    fn test_allowed_origins_reject_userinfo_and_invalid_ports() {
        assert_eq!(
            crate::allowed_origin::AllowedOrigin::try_from(String::from(
                constants_str::HTTPS_ADMIN_EXAMPLE_COM_WITH_USERINFO,
            )),
            Err(crate::allowed_origin_error::AllowedOriginError::Invalid)
        );
        assert_eq!(
            crate::allowed_origin::AllowedOrigin::try_from(String::from(
                constants_str::HTTPS_ADMIN_EXAMPLE_COM_WITH_INVALID_PORT,
            )),
            Err(crate::allowed_origin_error::AllowedOriginError::Invalid)
        );
    }

    #[test]
    fn test_origin_requires_exact_authority_without_path() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            http::header::ORIGIN,
            http::HeaderValue::from_static(constants_str::HTTPS_ADMIN_EXAMPLE_COM_PATH),
        );
        assert!(!bool::from(
            crate::resolve_request_origin_allowed::resolve_request_origin_allowed(
                crate::http_origin_headers_ref::HttpOriginHeadersRef::from(&headers),
                &allowed_origins(),
            )
        ));
    }

    #[test]
    fn test_referer_accepts_path_and_compares_case_insensitively() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            http::header::REFERER,
            http::HeaderValue::from_static(constants_str::HTTPS_ADMIN_EXAMPLE_COM_SETTINGS_UPPER),
        );
        assert!(bool::from(
            crate::resolve_request_origin_allowed::resolve_request_origin_allowed(
                crate::http_origin_headers_ref::HttpOriginHeadersRef::from(&headers),
                &allowed_origins(),
            )
        ));
    }
}
