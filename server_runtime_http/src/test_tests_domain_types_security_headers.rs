#[tokio::test]
async fn test_only_trusts_forwarded_proto_when_configured() {
    let make_request = || {
        axum::extract::Request::builder()
            .uri(constants_str::V1_TEST)
            .header(constants_str::X_FORWARDED_PROTO, constants_str::HTTPS)
            .body(axum::body::Body::empty())
            .expect(constants_str::DIAGNOSTIC_94149BDD)
    };
    let make_router = |trust| {
        let policy = crate::http_content_security_policy::HttpContentSecurityPolicy::try_from(
            constants_str::TEST_CONTENT_SECURITY_POLICY.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_ABF8CD24);
        axum::Router::from(
            crate::security_headers_layer::SecurityHeadersLayer::from(trust)
                .with_content_security_policy(policy)
                .apply(crate::axum_router::AxumRouter::from(
                    axum::Router::new().route(
                        constants_str::V1_TEST,
                        axum::routing::get(async || http::StatusCode::OK),
                    ),
                )),
        )
    };
    let ignored_response = tower::ServiceExt::oneshot(
        make_router(crate::forwarded_proto_trust::ForwardedProtoTrust::Ignore),
        make_request(),
    )
    .await
    .expect(constants_str::DIAGNOSTIC_8C89E84F);
    assert!(
        ignored_response
            .headers()
            .get(constants_str::STRICT_TRANSPORT_SECURITY)
            .is_none()
    );
    let trusted_response = tower::ServiceExt::oneshot(
        make_router(crate::forwarded_proto_trust::ForwardedProtoTrust::Trust),
        make_request(),
    )
    .await
    .expect(constants_str::DIAGNOSTIC_DB05C4BE);
    assert!(
        trusted_response
            .headers()
            .get(constants_str::STRICT_TRANSPORT_SECURITY)
            .is_some()
    );
    assert_eq!(
        trusted_response.headers().get(http::header::CACHE_CONTROL),
        Some(&http::HeaderValue::from_static(constants_str::NO_STORE))
    );
    assert_eq!(
        trusted_response
            .headers()
            .get(constants_str::X_CONTENT_TYPE_OPTIONS),
        Some(&http::HeaderValue::from_static(constants_str::NOSNIFF))
    );
    assert_eq!(
        trusted_response
            .headers()
            .get(constants_str::REFERRER_POLICY),
        Some(&http::HeaderValue::from_static(constants_str::SAME_ORIGIN))
    );
    assert_eq!(
        trusted_response
            .headers()
            .get(constants_str::CONTENT_SECURITY_POLICY_HEADER),
        Some(&http::HeaderValue::from_static(
            constants_str::TEST_CONTENT_SECURITY_POLICY
        ))
    );
}

#[tokio::test]
async fn test_marks_credentials_as_sensitive() {
    let router = axum::Router::from(
        crate::security_headers_layer::SecurityHeadersLayer::from(
            crate::forwarded_proto_trust::ForwardedProtoTrust::Ignore,
        )
        .apply(crate::axum_router::AxumRouter::from(
            axum::Router::new().route(
                constants_str::V1_TEST,
                axum::routing::get(async |headers: http::HeaderMap| {
                    assert!(
                        headers
                            .get(http::header::AUTHORIZATION)
                            .is_some_and(http::HeaderValue::is_sensitive)
                    );
                    (
                        [(
                            http::header::SET_COOKIE,
                            constants_str::TEST_SESSION_COOKIE_HEADER_VALUE,
                        )],
                        http::StatusCode::OK,
                    )
                }),
            ),
        )),
    );
    let response = tower::ServiceExt::oneshot(
        router,
        axum::extract::Request::builder()
            .uri(constants_str::V1_TEST)
            .header(
                http::header::AUTHORIZATION,
                constants_str::TEST_BEARER_AUTHORIZATION,
            )
            .body(axum::body::Body::empty())
            .expect(constants_str::DIAGNOSTIC_703AFFC9),
    )
    .await
    .expect(constants_str::DIAGNOSTIC_C975D44E);
    assert!(
        response
            .headers()
            .get(http::header::SET_COOKIE)
            .is_some_and(http::HeaderValue::is_sensitive)
    );
}

#[tokio::test]
async fn test_security_headers_mark_all_credential_values_and_preserve_unrelated_headers() {
    let names = [
        http::header::AUTHORIZATION,
        http::header::COOKIE,
        http::HeaderName::from_static(constants_str::X_CSRF_TOKEN_ALT),
    ];
    let mut prepared_request = names.iter().fold(
        axum::extract::Request::new(axum::body::Body::empty()),
        |mut request, name| {
            let _previous = request
                .headers_mut()
                .insert(name, http::HeaderValue::from_static(constants_str::X));
            let _appended = request.headers_mut().append(
                name,
                http::HeaderValue::from_static(constants_str::TEST_FIRST),
            );
            request
        },
    );
    let _previous_accept = prepared_request.headers_mut().insert(
        http::header::ACCEPT,
        http::HeaderValue::from_static(constants_str::X),
    );
    let inner = tower::service_fn(move |request: axum::extract::Request| {
        assert!(names.iter().all(|name| {
            let values = request.headers().get_all(name);
            values.iter().count() == 2usize
                && values
                    .iter()
                    .zip([constants_str::X, constants_str::TEST_FIRST])
                    .all(|(value, expected)| {
                        value.is_sensitive() && value.as_bytes() == expected.as_bytes()
                    })
        }));
        assert!(
            request
                .headers()
                .get(http::header::ACCEPT)
                .is_some_and(|value| {
                    !value.is_sensitive() && value.as_bytes() == constants_str::X.as_bytes()
                })
        );
        let mut response = axum::response::Response::new(axum::body::Body::from(constants_str::X));
        *response.status_mut() = http::StatusCode::CREATED;
        let _previous = response.headers_mut().insert(
            http::header::SET_COOKIE,
            http::HeaderValue::from_static(constants_str::X),
        );
        let _appended = response.headers_mut().append(
            http::header::SET_COOKIE,
            http::HeaderValue::from_static(constants_str::TEST_FIRST),
        );
        std::future::ready(Ok::<_, std::convert::Infallible>(response))
    });
    let result = tower::ServiceExt::oneshot(
        crate::security_headers_service::SecurityHeadersService::new(
            None,
            crate::forwarded_proto_trust::ForwardedProtoTrust::Ignore,
            inner,
        ),
        prepared_request,
    )
    .await;
    let response = match result {
        Ok(response) => response,
        Err(error) => match error {},
    };
    assert_eq!(response.status(), http::StatusCode::CREATED);
    let cookies = response.headers().get_all(http::header::SET_COOKIE);
    assert_eq!(cookies.iter().count(), 2usize);
    assert!(
        cookies
            .iter()
            .zip([constants_str::X, constants_str::TEST_FIRST])
            .all(|(value, expected)| {
                value.is_sensitive() && value.as_bytes() == expected.as_bytes()
            })
    );
    assert_eq!(
        response.headers().get(constants_str::X_FRAME_OPTIONS),
        Some(&http::HeaderValue::from_static(constants_str::DENY))
    );
    assert!(
        axum::body::to_bytes(response.into_body(), 1usize)
            .await
            .is_ok_and(|bytes| bytes.as_ref() == constants_str::X.as_bytes())
    );
}

#[tokio::test]
async fn test_security_headers_forwarded_proto_first_value_and_trust_policy() {
    let check = async |optional_header, expected| {
        let mut headers = http::HeaderMap::new();
        if let Some(header) = optional_header {
            let _previous = headers.insert(
                http::HeaderName::from_static(constants_str::X_FORWARDED_PROTO),
                header,
            );
        }
        let check_trust = async |trust, header_map, expected_hsts| {
            let mut request = axum::extract::Request::new(axum::body::Body::empty());
            *request.headers_mut() = header_map;
            let inner = tower::service_fn(|_request: axum::extract::Request| {
                std::future::ready(Ok::<_, std::convert::Infallible>(
                    axum::response::Response::new(axum::body::Body::empty()),
                ))
            });
            let result = tower::ServiceExt::oneshot(
                crate::security_headers_service::SecurityHeadersService::new(None, trust, inner),
                request,
            )
            .await;
            let response = match result {
                Ok(response) => response,
                Err(error) => match error {},
            };
            let expected_header = bool::then_some(
                expected_hsts,
                http::HeaderValue::from_static(constants_str::MAX_AGE_31536000_INCLUDESUBDOMAINS),
            );
            assert_eq!(
                response
                    .headers()
                    .get(constants_str::STRICT_TRANSPORT_SECURITY),
                expected_header.as_ref()
            );
        };
        check_trust(
            crate::forwarded_proto_trust::ForwardedProtoTrust::Ignore,
            headers.clone(),
            false,
        )
        .await;
        check_trust(
            crate::forwarded_proto_trust::ForwardedProtoTrust::Trust,
            headers,
            expected,
        )
        .await;
    };
    let fixtures_match = if let (Ok(binary), Ok(https_first), Ok(http_first)) = (
        http::HeaderValue::from_bytes(&[0xffu8]),
        http::HeaderValue::from_str(&format!(
            " {},{}",
            constants_str::HTTPS.to_ascii_uppercase(),
            constants_str::HTTP
        )),
        http::HeaderValue::from_str(&format!("{},{}", constants_str::HTTP, constants_str::HTTPS)),
    ) {
        check(None, false).await;
        check(
            Some(http::HeaderValue::from_static(constants_str::HTTPS)),
            true,
        )
        .await;
        check(
            Some(http::HeaderValue::from_static(constants_str::HTTP)),
            false,
        )
        .await;
        check(Some(binary), false).await;
        check(Some(https_first), true).await;
        check(Some(http_first), false).await;
        true
    } else {
        false
    };
    assert!(fixtures_match);
}

#[tokio::test]
async fn test_security_headers_preserve_response_and_apply_exact_api_cache_and_csp_policy() {
    let check = async |path, api, configured| {
        let inner = tower::service_fn(|_request: axum::extract::Request| {
            let mut response =
                axum::response::Response::new(axum::body::Body::from(constants_str::X));
            *response.status_mut() = http::StatusCode::BAD_REQUEST;
            let _previous_cache = response.headers_mut().insert(
                http::header::CACHE_CONTROL,
                http::HeaderValue::from_static(constants_str::X),
            );
            let _previous_csp = response.headers_mut().insert(
                http::HeaderName::from_static(constants_str::CONTENT_SECURITY_POLICY_HEADER),
                http::HeaderValue::from_static(constants_str::X),
            );
            let _previous_hsts = response.headers_mut().insert(
                http::HeaderName::from_static(constants_str::STRICT_TRANSPORT_SECURITY),
                http::HeaderValue::from_static(constants_str::X),
            );
            std::future::ready(Ok::<_, std::convert::Infallible>(response))
        });
        let converted = crate::http_content_security_policy::HttpContentSecurityPolicy::try_from(
            constants_str::TEST_CONTENT_SECURITY_POLICY.to_owned(),
        );
        let fixture_matches = if let Ok(policy) = converted {
            let mut request = axum::extract::Request::new(axum::body::Body::empty());
            *request.uri_mut() = http::Uri::from_static(path);
            let result = tower::ServiceExt::oneshot(
                crate::security_headers_service::SecurityHeadersService::new(
                    bool::then_some(configured, policy),
                    crate::forwarded_proto_trust::ForwardedProtoTrust::Ignore,
                    inner,
                ),
                request,
            )
            .await;
            let response = match result {
                Ok(response) => response,
                Err(error) => match error {},
            };
            assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
            assert_eq!(
                response.headers().get(http::header::CACHE_CONTROL),
                Some(&http::HeaderValue::from_static(if api {
                    constants_str::NO_STORE
                } else {
                    constants_str::X
                }))
            );
            assert_eq!(
                response
                    .headers()
                    .get(constants_str::CONTENT_SECURITY_POLICY_HEADER),
                Some(&http::HeaderValue::from_static(if configured {
                    constants_str::TEST_CONTENT_SECURITY_POLICY
                } else {
                    constants_str::X
                }))
            );
            assert_eq!(
                response
                    .headers()
                    .get(constants_str::STRICT_TRANSPORT_SECURITY),
                Some(&http::HeaderValue::from_static(constants_str::X))
            );
            assert_eq!(
                response.headers().get(constants_str::X_FRAME_OPTIONS),
                Some(&http::HeaderValue::from_static(constants_str::DENY))
            );
            axum::body::to_bytes(response.into_body(), 1usize)
                .await
                .is_ok_and(|bytes| bytes.as_ref() == constants_str::X.as_bytes())
        } else {
            false
        };
        assert!(fixture_matches);
    };
    check(constants_str::SLASH, false, false).await;
    check(constants_str::SLASH, false, true).await;
    check(constants_str::V1_SLASH.trim_end_matches('/'), false, false).await;
    check(constants_str::V1_SLASH.trim_end_matches('/'), false, true).await;
    check(constants_str::V1_TEST, true, false).await;
    check(constants_str::V1_TEST, true, true).await;
}
