#[tokio::test]
async fn only_trusts_forwarded_proto_when_configured() {
    let make_request = || {
        axum::extract::Request::builder()
            .uri(constants_str::catalog::V1_TEST)
            .header(
                constants_str::catalog::X_FORWARDED_PROTO,
                constants_str::catalog::HTTPS,
            )
            .body(axum::body::Body::empty())
            .expect("94149bdd only_trusts_forwarded_proto_when_configured invariant must hold")
    };
    let make_router = |trust| {
        let policy = crate::http_content_security_policy::HttpContentSecurityPolicy::try_from(
            constants_str::test_fixtures::TEST_CONTENT_SECURITY_POLICY.to_owned(),
        )
        .expect("abf8cd24 only_trusts_forwarded_proto_when_configured invariant must hold");
        axum::Router::from(
            crate::security_headers_layer::SecurityHeadersLayer::from(trust)
                .with_content_security_policy(policy)
                .apply(crate::axum_router::AxumRouter::from(
                    axum::Router::new().route(
                        constants_str::catalog::V1_TEST,
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
    .expect("8c89e84f only_trusts_forwarded_proto_when_configured invariant must hold");
    assert!(
        ignored_response
            .headers()
            .get(constants_str::catalog::STRICT_TRANSPORT_SECURITY)
            .is_none()
    );
    let trusted_response = tower::ServiceExt::oneshot(
        make_router(crate::forwarded_proto_trust::ForwardedProtoTrust::Trust),
        make_request(),
    )
    .await
    .expect("db05c4be only_trusts_forwarded_proto_when_configured invariant must hold");
    assert!(
        trusted_response
            .headers()
            .get(constants_str::catalog::STRICT_TRANSPORT_SECURITY)
            .is_some()
    );
    assert_eq!(
        trusted_response.headers().get(http::header::CACHE_CONTROL),
        Some(&http::HeaderValue::from_static(
            constants_str::catalog::NO_STORE
        ))
    );
    assert_eq!(
        trusted_response
            .headers()
            .get(constants_str::catalog::X_CONTENT_TYPE_OPTIONS),
        Some(&http::HeaderValue::from_static(
            constants_str::catalog::NOSNIFF
        ))
    );
    assert_eq!(
        trusted_response
            .headers()
            .get(constants_str::catalog::REFERRER_POLICY),
        Some(&http::HeaderValue::from_static(
            constants_str::catalog::SAME_ORIGIN
        ))
    );
    assert_eq!(
        trusted_response
            .headers()
            .get(constants_str::test_fixtures::CONTENT_SECURITY_POLICY_HEADER),
        Some(&http::HeaderValue::from_static(
            constants_str::test_fixtures::TEST_CONTENT_SECURITY_POLICY
        ))
    );
}

#[tokio::test]
async fn marks_credentials_as_sensitive() {
    let router = axum::Router::from(
        crate::security_headers_layer::SecurityHeadersLayer::from(
            crate::forwarded_proto_trust::ForwardedProtoTrust::Ignore,
        )
        .apply(crate::axum_router::AxumRouter::from(
            axum::Router::new().route(
                constants_str::catalog::V1_TEST,
                axum::routing::get(async |headers: http::HeaderMap| {
                    assert!(
                        headers
                            .get(http::header::AUTHORIZATION)
                            .is_some_and(http::HeaderValue::is_sensitive)
                    );
                    (
                        [(
                            http::header::SET_COOKIE,
                            constants_str::test_fixtures::TEST_SESSION_COOKIE_HEADER_VALUE,
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
            .uri(constants_str::catalog::V1_TEST)
            .header(
                http::header::AUTHORIZATION,
                constants_str::test_fixtures::TEST_BEARER_AUTHORIZATION,
            )
            .body(axum::body::Body::empty())
            .expect("703affc9 marks_credentials_as_sensitive invariant must hold"),
    )
    .await
    .expect("c975d44e marks_credentials_as_sensitive invariant must hold");
    assert!(
        response
            .headers()
            .get(http::header::SET_COOKIE)
            .is_some_and(http::HeaderValue::is_sensitive)
    );
}
