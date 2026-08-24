#[test]
fn rate_limit_scopes_are_distinct() {
    let scopes = [
        super::rate_limit::AdminRateLimitScope::AuditExport,
        super::rate_limit::AdminRateLimitScope::Mutation,
        super::rate_limit::AdminRateLimitScope::RefreshIp,
        super::rate_limit::AdminRateLimitScope::SignInIp,
        super::rate_limit::AdminRateLimitScope::SignInIpLogin,
    ]
    .map(super::rate_limit::AdminRateLimitScope::as_str);
    assert_eq!(
        scopes[0].as_ref(),
        constants_str::SERVER_ADMIN_RATE_LIMIT_AUDIT_EXPORT
    );
    let unique = scopes.into_iter().collect::<std::collections::HashSet<_>>();
    assert_eq!(unique.len(), 5usize);
}
#[test]
fn rate_limited_error_includes_retry_after_header() {
    let response = axum::response::IntoResponse::into_response(super::AdminError::RateLimited);
    assert_eq!(response.status(), http::StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(
        response.headers().get(http::header::RETRY_AFTER),
        Some(&http::HeaderValue::from_static("60")),
    );
    assert!(
        response
            .extensions()
            .get::<server_runtime_http::HttpErrorDiagnostic>()
            .is_none()
    );
}
#[test]
fn server_error_response_preserves_http_diagnostic() {
    let response = axum::response::IntoResponse::into_response(super::AdminError::pg(
        super::super::SqlxAdminError::from(sqlx::Error::RowNotFound),
    ));
    assert_eq!(response.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    assert!(
        response
            .extensions()
            .get::<server_runtime_http::HttpErrorDiagnostic>()
            .is_some()
    );
    assert_eq!(
        response.headers().get(http::header::CONTENT_TYPE),
        Some(&http::HeaderValue::from_static(
            constants_str::APPLICATION_PROBLEM_PLUS_JSON
        ))
    );
    let body = futures::executor::block_on(axum::body::to_bytes(response.into_body(), 16_384usize))
        .expect("8770f4d3 server_error_response_preserves_http_diagnostic invariant must hold");
    let contract_problem = serde_json::from_slice::<frontend_contract::ApiProblem>(&body)
        .expect("4f705ab8 server_error_response_preserves_http_diagnostic invariant must hold");
    assert_eq!(
        contract_problem.kind(),
        frontend_contract::ApiProblemKind::Internal
    );
    let problem = serde_json::from_slice::<serde_json::Value>(&body)
        .expect("1e7ec09d server_error_response_preserves_http_diagnostic invariant must hold");
    [
        "location",
        "error_location",
        "backtrace",
        "error_chain",
        "span_trace",
    ]
    .into_iter()
    .for_each(|private_field| {
        assert!(problem.get(private_field).is_none());
    });
}
#[test]
fn session_context_hash_is_bound_to_peer_and_user_agent() {
    let mut first_headers = http::HeaderMap::new();
    let _previous_user_agent = first_headers.insert(
        http::header::USER_AGENT,
        http::HeaderValue::from_static(constants_str::ADMIN_CLIENT_1),
    );
    let first_peer = super::AdminPeerAddr::from(super::super::StdAdminSocketAddr::from(
        constants_str::VALUE_192_0_2_10_443
            .parse::<std::net::SocketAddr>()
            .expect(
                "f133a4ca session_context_hash_is_bound_to_peer_and_user_agent invariant must hold",
            ),
    ));
    let same_context_hash = super::session_context_hash(
        super::super::HttpAdminHeaderMapRef::from(&first_headers),
        first_peer,
    )
    .expect("14f0aa2d session_context_hash_is_bound_to_peer_and_user_agent invariant must hold");
    let repeated_context_hash = super::session_context_hash(
        super::super::HttpAdminHeaderMapRef::from(&first_headers),
        first_peer,
    )
    .expect("998805c8 session_context_hash_is_bound_to_peer_and_user_agent invariant must hold");
    assert_eq!(
        secrecy::ExposeSecret::expose_secret(same_context_hash.0.as_ref()),
        secrecy::ExposeSecret::expose_secret(repeated_context_hash.0.as_ref()),
    );
    let other_peer = super::AdminPeerAddr::from(super::super::StdAdminSocketAddr::from(
        constants_str::VALUE_192_0_2_11_443
            .parse::<std::net::SocketAddr>()
            .expect(
                "5a831a2f session_context_hash_is_bound_to_peer_and_user_agent invariant must hold",
            ),
    ));
    let other_peer_hash = super::session_context_hash(
        super::super::HttpAdminHeaderMapRef::from(&first_headers),
        other_peer,
    )
    .expect("0803469a session_context_hash_is_bound_to_peer_and_user_agent invariant must hold");
    assert_ne!(
        secrecy::ExposeSecret::expose_secret(same_context_hash.0.as_ref()),
        secrecy::ExposeSecret::expose_secret(other_peer_hash.0.as_ref()),
    );
    let mut other_headers = http::HeaderMap::new();
    let _previous_other_user_agent = other_headers.insert(
        http::header::USER_AGENT,
        http::HeaderValue::from_static(constants_str::ADMIN_CLIENT_2),
    );
    let other_user_agent_hash = super::session_context_hash(
        super::super::HttpAdminHeaderMapRef::from(&other_headers),
        first_peer,
    )
    .expect("90ce47ee session_context_hash_is_bound_to_peer_and_user_agent invariant must hold");
    assert_ne!(
        secrecy::ExposeSecret::expose_secret(same_context_hash.0.as_ref()),
        secrecy::ExposeSecret::expose_secret(other_user_agent_hash.0.as_ref()),
    );
}
#[test]
fn audit_resource_identifier_uses_target_identifier() {
    assert_eq!(
        super::AdminAuditResourceId::User(crate::AdminUserId::try_from(42i64).expect(
            "423b91b9 audit_resource_identifier_uses_target_identifier invariant must hold"
        ),)
        .value()
        .as_ref(),
        "42"
    );
    assert_eq!(
        super::AdminAuditResourceId::Role(crate::AdminRoleId::try_from(7i64).expect(
            "af8df9d2 audit_resource_identifier_uses_target_identifier invariant must hold"
        ),)
        .value()
        .as_ref(),
        "7"
    );
    assert_eq!(
        super::AdminAuditResourceId::SystemSettings.value().as_ref(),
        "1"
    );
}
#[test]
fn open_api_contains_auth_and_user_security_contracts() {
    frontend_contract_validation::validate_openapi_schema_references(
        &utoipa::openapi::OpenApi::from(super::open_api()),
    )
    .expect("2151641d open_api_contains_auth_and_user_security_contracts invariant must hold");
    let document = serde_json::to_value(utoipa::openapi::OpenApi::from(super::open_api()))
        .expect("869d28d7 open_api_contains_auth_and_user_security_contracts invariant must hold");
    let paths = document
        .get(constants_str::PATHS)
        .and_then(serde_json::Value::as_object)
        .expect("6e15edec open_api_contains_auth_and_user_security_contracts invariant must hold");
    assert_eq!(paths.len(), 22usize);
    assert!(!paths.contains_key("/auth/mfa"));
    assert!(!paths.contains_key("/auth/mfa/enroll"));
    assert!(!paths.contains_key("/auth/mfa/confirm"));
    assert!(!paths.contains_key("/auth/mfa/step-up"));
    let documented_route_contracts = paths
        .iter()
        .flat_map(|(path, path_item)| {
            path_item
                .as_object()
                .into_iter()
                .flat_map(|operation_map| operation_map.iter())
                .map(move |(method, operation)| {
                    (
                        method.to_owned(),
                        operation
                            .get(constants_str::OPERATION_ID_JSON)
                            .and_then(serde_json::Value::as_str)
                            .expect("4252acc8 open_api_contains_auth_and_user_security_contracts invariant must hold")
                            .to_owned(),
                        path.to_owned(),
                    )
                })
        })
        .collect::<std::collections::BTreeSet<_>>();
    let contracted_route_contracts = <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::coverage_descriptors()
            .as_ref()
            .iter()
            .copied()
            .map(|descriptor| {
                let metadata = descriptor.metadata();
                (
                    metadata.method().as_ref().to_ascii_lowercase(),
                    metadata.openapi_operation_id().as_ref().to_owned(),
                    metadata.path().as_ref().to_owned(),
                )
            })
            .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(documented_route_contracts, contracted_route_contracts);
    assert!(paths.contains_key("/auth/sign_in"));
    assert!(paths.contains_key("/auth/sessions/{session_id}"));
    assert!(paths.contains_key("/users/{user_id}/password"));
    assert!(paths.contains_key("/roles/{role_id}/permissions"));
    assert!(paths.contains_key("/permissions"));
    assert!(paths.contains_key("/audit_log"));
    assert!(paths.contains_key("/system_settings"));
    assert_eq!(
        document
            .pointer(constants_str::ADMIN_OPENAPI_SIGN_IN_OPERATION_ID_POINTER)
            .and_then(serde_json::Value::as_str),
        Some(
            <server_admin_contract::AdminSignInRoute as frontend_contract::TypedRoute>::metadata()
                .openapi_operation_id()
                .as_ref()
        ),
    );
    assert_eq!(
        document
            .pointer(constants_str::ADMIN_OPENAPI_REFRESH_OPERATION_ID_POINTER)
            .and_then(serde_json::Value::as_str),
        Some(
            <server_admin_contract::AdminRefreshRoute as frontend_contract::TypedRoute>::metadata()
                .openapi_operation_id()
                .as_ref()
        ),
    );
    assert_eq!(
        document
            .pointer(constants_str::ADMIN_OPENAPI_ME_OPERATION_ID_POINTER)
            .and_then(serde_json::Value::as_str),
        Some(
            <server_admin_contract::AdminMeRoute as frontend_contract::TypedRoute>::metadata()
                .openapi_operation_id()
                .as_ref()
        ),
    );
    assert!(
        paths
            .values()
            .all(|path| path
                .as_object()
                .is_some_and(|operations| operations.values().all(|operation| operation
                    .pointer("/responses/429/headers/Retry-After")
                    .is_some())))
    );
    assert!(
        document
            .pointer("/components/securitySchemes/admin_cookie")
            .is_some()
    );
    assert!(
        document
            .pointer("/components/securitySchemes/admin_csrf")
            .is_some()
    );
    assert_eq!(
        document
            .pointer("/components/schemas/AdminPassword/writeOnly")
            .and_then(serde_json::Value::as_bool),
        Some(true),
    );
    let expected_body_limit_description = format!(
            "{}{}",
            constants_str::OPENAPI_REQUEST_BODY_MAXIMUM_BYTES_PREFIX,
            <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::body_limit()
                .expect("be105d90 open_api_contains_auth_and_user_security_contracts invariant must hold")
                .get()
        );
    let request_body_descriptions = paths
        .values()
        .filter_map(|path| path.as_object())
        .flat_map(|operations| operations.values())
        .filter_map(|operation| {
            operation.pointer(constants_str::OPENAPI_REQUEST_BODY_DESCRIPTION_POINTER)
        })
        .filter_map(serde_json::Value::as_str)
        .collect::<Vec<_>>();
    assert!(!request_body_descriptions.is_empty());
    assert!(
        request_body_descriptions
            .into_iter()
            .all(|description| description == expected_body_limit_description)
    );
}
