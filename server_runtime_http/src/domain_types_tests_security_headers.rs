#[tokio::test]
async fn only_trusts_forwarded_proto_when_configured() {
    let make_request = || {
        axum::extract::Request::builder()
            .uri(constants_str::V1_TEST)
            .header(constants_str::X_FORWARDED_PROTO, constants_str::HTTPS)
            .body(axum::body::Body::empty())
            .expect("94149bdd only_trusts_forwarded_proto_when_configured invariant must hold")
    };
    let make_router = |trust| {
        let policy = crate::HttpContentSecurityPolicy::try_from(
            constants_str::TEST_CONTENT_SECURITY_POLICY.to_owned(),
        )
        .expect("abf8cd24 only_trusts_forwarded_proto_when_configured invariant must hold");
        axum::Router::from(
            crate::SecurityHeadersLayer::from(trust)
                .with_content_security_policy(policy)
                .apply(crate::AxumRouter::from(axum::Router::new().route(
                    constants_str::V1_TEST,
                    axum::routing::get(async || http::StatusCode::OK),
                ))),
        )
    };
    let ignored_response = tower::ServiceExt::oneshot(
        make_router(crate::ForwardedProtoTrust::Ignore),
        make_request(),
    )
    .await
    .expect("8c89e84f only_trusts_forwarded_proto_when_configured invariant must hold");
    assert!(
        ignored_response
            .headers()
            .get(constants_str::STRICT_TRANSPORT_SECURITY)
            .is_none()
    );
    let trusted_response = tower::ServiceExt::oneshot(
        make_router(crate::ForwardedProtoTrust::Trust),
        make_request(),
    )
    .await
    .expect("db05c4be only_trusts_forwarded_proto_when_configured invariant must hold");
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
async fn marks_credentials_as_sensitive() {
    let router = axum::Router::from(
        crate::SecurityHeadersLayer::from(crate::ForwardedProtoTrust::Ignore).apply(
            crate::AxumRouter::from(axum::Router::new().route(
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
            )),
        ),
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
