#[cfg(test)]
mod tests {
    #[test]
    fn test_origin_header_precedence_over_valid_invalid_and_duplicate_referers() {
        let allowed = allowed_origins();
        assert!(
            http::HeaderValue::from_bytes(&[0xffu8]).is_ok_and(|opaque| {
                let valid_origin =
                    http::HeaderValue::from_static(constants_str::HTTPS_ADMIN_EXAMPLE_COM);
                let valid_referer =
                    http::HeaderValue::from_static(constants_str::HTTPS_ADMIN_EXAMPLE_COM_PATH);
                let invalid = http::HeaderValue::from_static(constants_str::HTTP_LOCALHOST);
                let origins = [
                    (None, false),
                    (Some(valid_origin), true),
                    (Some(invalid.clone()), false),
                    (Some(opaque.clone()), false),
                    (
                        Some(http::HeaderValue::from_static(
                            constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
                        )),
                        false,
                    ),
                ];
                let referers = [
                    (vec![], false),
                    (vec![valid_referer.clone()], true),
                    (vec![invalid], false),
                    (vec![opaque], false),
                    (vec![valid_referer.clone(), valid_referer], false),
                ];
                origins.iter().all(|(origin, origin_allowed)| {
                    referers.iter().all(|(referer_values, referer_allowed)| {
                        let mut headers = http::HeaderMap::new();
                        if let Some(origin_value) = origin {
                            let _previous =
                                headers.insert(http::header::ORIGIN, origin_value.clone());
                        }
                        let _appended_count =
                            referer_values.iter().fold(0usize, |count, referer| {
                                let _appended =
                                    headers.append(http::header::REFERER, referer.clone());
                                count + 1usize
                            });
                        let expected = origin
                            .as_ref()
                            .map_or(*referer_allowed, |_| *origin_allowed);
                        bool::from(
                            crate::resolve_request_origin_allowed::resolve_request_origin_allowed(
                                crate::http_origin_headers_ref::HttpOriginHeadersRef::from(
                                    &headers,
                                ),
                                &allowed,
                            ),
                        ) == expected
                    })
                })
            })
        );
    }

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
    fn test_origin_matches_configured_default_ports() {
        let cases = [
            (constants_str::HTTP_LOCALHOST, 80u16),
            (constants_str::HTTPS_ADMIN_EXAMPLE_COM, 443u16),
        ];
        assert!(cases.into_iter().all(|(origin, port)| {
            let configured = format!("{origin}:{port}");
            let parsed_allowed_origins =
                crate::allowed_origins::AllowedOrigins::try_from(vec![configured]);
            let mut headers = http::HeaderMap::new();
            let _previous =
                headers.insert(http::header::ORIGIN, http::HeaderValue::from_static(origin));
            parsed_allowed_origins.is_ok_and(|allowed_origins| {
                bool::from(
                    crate::resolve_request_origin_allowed::resolve_request_origin_allowed(
                        crate::http_origin_headers_ref::HttpOriginHeadersRef::from(&headers),
                        &allowed_origins,
                    ),
                )
            })
        }));
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

    #[test]
    fn test_origin_rejects_duplicate_headers() {
        let mut headers = http::HeaderMap::new();
        let _inserted_referer = headers.append(
            http::header::REFERER,
            http::HeaderValue::from_static(constants_str::HTTPS_ADMIN_EXAMPLE_COM_PATH),
        );
        let _inserted_first_origin = headers.append(
            http::header::ORIGIN,
            http::HeaderValue::from_static(constants_str::HTTPS_ADMIN_EXAMPLE_COM),
        );
        let _inserted_second_origin = headers.append(
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
    fn test_referer_rejects_duplicate_headers() {
        let mut headers = http::HeaderMap::new();
        let _inserted_first_referer = headers.append(
            http::header::REFERER,
            http::HeaderValue::from_static(constants_str::HTTPS_ADMIN_EXAMPLE_COM_PATH),
        );
        let _inserted_second_referer = headers.append(
            http::header::REFERER,
            http::HeaderValue::from_static(constants_str::HTTPS_ADMIN_EXAMPLE_COM_SETTINGS_UPPER),
        );
        assert!(!bool::from(
            crate::resolve_request_origin_allowed::resolve_request_origin_allowed(
                crate::http_origin_headers_ref::HttpOriginHeadersRef::from(&headers),
                &allowed_origins(),
            )
        ));
    }
    #[test]
    fn test_origin_and_referer_matching_preserves_exact_authority_and_suffix_policy() {
        let allowed = allowed_origins();
        assert!([
            http::header::ORIGIN,
            http::header::REFERER,
        ].into_iter().all(|name| {
            [
                (constants_str::HTTPS_ADMIN_EXAMPLE_COM.to_owned(), true),
                (constants_str::HTTPS_ADMIN_EXAMPLE_COM.to_ascii_uppercase(), true),
                (format!(" {} ", constants_str::HTTPS_ADMIN_EXAMPLE_COM), true),
                (format!("{}{}", constants_str::HTTPS_ADMIN_EXAMPLE_COM, constants_str::X), false),
                (format!("{}:{}", constants_str::HTTPS_ADMIN_EXAMPLE_COM, 444u16), false),
                (constants_str::HTTPS_ADMIN_EXAMPLE_COM.replacen(constants_str::HTTPS, constants_str::HTTP, 1usize), false),
                (format!("{}/{}", constants_str::HTTPS_ADMIN_EXAMPLE_COM, constants_str::X), name == http::header::REFERER),
                (format!("{}?{}", constants_str::HTTPS_ADMIN_EXAMPLE_COM, constants_str::X), name == http::header::REFERER),
                (format!("{}#{}", constants_str::HTTPS_ADMIN_EXAMPLE_COM, constants_str::X), name == http::header::REFERER),
            ].into_iter().all(|(text, expected)| {
                http::HeaderValue::from_str(&text).is_ok_and(|value| {
                    let mut headers = http::HeaderMap::new();
                    let _previous = headers.insert(&name, value);
                    bool::from(crate::resolve_request_origin_allowed::resolve_request_origin_allowed(
                        crate::http_origin_headers_ref::HttpOriginHeadersRef::from(&headers),
                        &allowed,
                    )) == expected
                })
            })
        }));
    }

    #[test]
    fn test_allowed_origin_list_boundaries_preserve_last_entry_matching() {
        assert!([0usize, 1usize, 127usize, 128usize, 129usize].into_iter().all(|count| {
            let values = (0usize..count).map(|index| {
                if index + 1usize == count {
                    constants_str::HTTPS_ADMIN_EXAMPLE_COM.to_owned()
                } else {
                    constants_str::HTTP_LOCALHOST.to_owned()
                }
            }).collect::<Vec<_>>();
            let result = crate::allowed_origins::AllowedOrigins::try_from(values);
            if count == 129usize {
                result == Err(crate::allowed_origins_error::AllowedOriginsError::Invalid)
            } else {
                result.is_ok_and(|allowed| {
                    let mut headers = http::HeaderMap::new();
                    let _previous = headers.insert(http::header::ORIGIN, http::HeaderValue::from_static(constants_str::HTTPS_ADMIN_EXAMPLE_COM));
                    bool::from(crate::resolve_request_origin_allowed::resolve_request_origin_allowed(
                        crate::http_origin_headers_ref::HttpOriginHeadersRef::from(&headers),
                        &allowed,
                    )) == (count != 0usize)
                })
            }
        }));
    }
}
