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
