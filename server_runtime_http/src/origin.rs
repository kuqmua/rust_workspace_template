#[cfg(test)]
mod tests {
    fn allowed_origins() -> crate::allowed_origins::AllowedOrigins {
        crate::allowed_origins::AllowedOrigins::try_from(vec![String::from(
            constants_str::catalog::HTTPS_ADMIN_EXAMPLE_COM,
        )])
        .expect("782d2bed allowed_origins invariant must hold")
    }

    #[test]
    fn allowed_origins_reject_oversized_lists() {
        let values = vec![String::from(constants_str::catalog::HTTPS_ADMIN_EXAMPLE_COM); 129usize];
        assert_eq!(
            crate::allowed_origins::AllowedOrigins::try_from(values),
            Err(crate::allowed_origins_error::AllowedOriginsError::Invalid)
        );
    }

    #[test]
    fn allowed_origins_reject_userinfo_and_invalid_ports() {
        assert_eq!(
            crate::allowed_origin::AllowedOrigin::try_from(String::from(
                constants_str::catalog::HTTPS_ADMIN_EXAMPLE_COM_WITH_USERINFO,
            )),
            Err(crate::allowed_origin_error::AllowedOriginError::Invalid)
        );
        assert_eq!(
            crate::allowed_origin::AllowedOrigin::try_from(String::from(
                constants_str::catalog::HTTPS_ADMIN_EXAMPLE_COM_WITH_INVALID_PORT,
            )),
            Err(crate::allowed_origin_error::AllowedOriginError::Invalid)
        );
    }

    #[test]
    fn origin_requires_exact_authority_without_path() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            http::header::ORIGIN,
            http::HeaderValue::from_static(constants_str::catalog::HTTPS_ADMIN_EXAMPLE_COM_PATH),
        );
        assert!(!bool::from(
            crate::resolve_request_origin_allowed::resolve_request_origin_allowed(
                crate::http_origin_headers_ref::HttpOriginHeadersRef::from(&headers),
                &allowed_origins(),
            )
        ));
    }

    #[test]
    fn referer_accepts_path_and_compares_case_insensitively() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            http::header::REFERER,
            http::HeaderValue::from_static(
                constants_str::catalog::HTTPS_ADMIN_EXAMPLE_COM_SETTINGS_UPPER,
            ),
        );
        assert!(bool::from(
            crate::resolve_request_origin_allowed::resolve_request_origin_allowed(
                crate::http_origin_headers_ref::HttpOriginHeadersRef::from(&headers),
                &allowed_origins(),
            )
        ));
    }
}

// Root-owned module compatibility wrappers.
mod allow_origin_suffix {}
mod allowed_origin {}
mod allowed_origin_error {}
mod allowed_origins {}
mod allowed_origins_error {}
mod http_origin_authority_text {}
mod http_origin_headers_ref {}
mod http_origin_scheme_text {}
mod http_origin_text_ref {}
mod parsed_http_origin_ref {}
mod request_origin_allowed {}
mod request_origin_value_is_allowed {}
mod resolve_request_origin_allowed {}
