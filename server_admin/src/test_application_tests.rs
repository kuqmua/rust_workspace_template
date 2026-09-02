#[test]
fn test_rate_limit_scopes_are_distinct() {
    let scopes = [
        crate::admin_rate_limit_scope::AdminRateLimitScope::AuditExport,
        crate::admin_rate_limit_scope::AdminRateLimitScope::Mutation,
        crate::admin_rate_limit_scope::AdminRateLimitScope::RefreshIp,
        crate::admin_rate_limit_scope::AdminRateLimitScope::SignInIp,
        crate::admin_rate_limit_scope::AdminRateLimitScope::SignInIpLogin,
    ]
    .map(crate::admin_rate_limit_scope::AdminRateLimitScope::as_str);
    assert_eq!(
        scopes[0].as_ref(),
        constants_str::SERVER_ADMIN_RATE_LIMIT_AUDIT_EXPORT
    );
    let unique = scopes.into_iter().collect::<std::collections::HashSet<_>>();
    assert_eq!(unique.len(), 5usize);
}
#[test]
fn test_rate_limited_error_includes_retry_after_header() {
    let response =
        axum::response::IntoResponse::into_response(crate::admin_error::AdminError::RateLimited);
    assert_eq!(response.status(), http::StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(
        response.headers().get(http::header::RETRY_AFTER),
        Some(&http::HeaderValue::from_static(constants_str::VALUE_60)),
    );
    assert!(
        response
            .extensions()
            .get::<server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic>()
            .is_none()
    );
}
#[test]
fn test_server_error_response_preserves_http_diagnostic() {
    let response =
        axum::response::IntoResponse::into_response(crate::admin_error::AdminError::postgresql(
            crate::sqlx_admin_error::SqlxAdminError::from(sqlx::Error::RowNotFound),
        ));
    assert_eq!(response.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    assert!(
        response
            .extensions()
            .get::<server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic>()
            .is_some()
    );
    assert_eq!(
        response.headers().get(http::header::CONTENT_TYPE),
        Some(&http::HeaderValue::from_static(
            constants_str::APPLICATION_PROBLEM_PLUS_JSON
        ))
    );
    let body = futures::executor::block_on(axum::body::to_bytes(response.into_body(), 16_384usize))
        .expect(constants_str::DIAGNOSTIC_8770F4D3);
    let contract_problem =
        serde_json::from_slice::<frontend_contract::api_problem::ApiProblem>(&body)
            .expect(constants_str::DIAGNOSTIC_4F705AB8);
    assert_eq!(
        contract_problem.kind(),
        frontend_contract::api_problem_kind::ApiProblemKind::Internal
    );
    let problem = serde_json::from_slice::<serde_json::Value>(&body)
        .expect(constants_str::DIAGNOSTIC_1E7EC09D);
    [
        constants_str::LOCATION_ALT,
        constants_str::VALUE_31755A3B,
        constants_str::VALUE_265EE18A,
        constants_str::VALUE_4C133E94,
        constants_str::VALUE_86846B4A,
    ]
    .into_iter()
    .for_each(|private_field| {
        assert!(problem.get(private_field).is_none());
    });
}
#[test]
fn test_session_context_hash_is_bound_to_peer_and_user_agent() {
    let mut first_headers = http::HeaderMap::new();
    let _previous_user_agent = first_headers.insert(
        http::header::USER_AGENT,
        http::HeaderValue::from_static(constants_str::ADMIN_CLIENT_1),
    );
    let first_peer = crate::admin_peer_addr::AdminPeerAddr::from(
        server_admin_core::admin_socket_addr::AdminSocketAddr::from(
            constants_str::VALUE_192_0_2_10_443
                .parse::<std::net::SocketAddr>()
                .expect(constants_str::DIAGNOSTIC_F133A4CA),
        ),
    );
    let same_context_hash =
        crate::authorization_session_context_hash::authorization_session_context_hash(
            crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(&first_headers),
            first_peer,
        )
        .expect(constants_str::DIAGNOSTIC_14F0AA2D);
    let repeated_context_hash =
        crate::authorization_session_context_hash::authorization_session_context_hash(
            crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(&first_headers),
            first_peer,
        )
        .expect(constants_str::DIAGNOSTIC_998805C8);
    assert_eq!(
        same_context_hash.expose().as_ref(),
        repeated_context_hash.expose().as_ref(),
    );
    let other_peer = crate::admin_peer_addr::AdminPeerAddr::from(
        server_admin_core::admin_socket_addr::AdminSocketAddr::from(
            constants_str::VALUE_192_0_2_11_443
                .parse::<std::net::SocketAddr>()
                .expect(constants_str::DIAGNOSTIC_5A831A2F),
        ),
    );
    let other_peer_hash =
        crate::authorization_session_context_hash::authorization_session_context_hash(
            crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(&first_headers),
            other_peer,
        )
        .expect(constants_str::DIAGNOSTIC_0803469A);
    assert_ne!(
        same_context_hash.expose().as_ref(),
        other_peer_hash.expose().as_ref(),
    );
    let mut other_headers = http::HeaderMap::new();
    let _previous_other_user_agent = other_headers.insert(
        http::header::USER_AGENT,
        http::HeaderValue::from_static(constants_str::ADMIN_CLIENT_2),
    );
    let other_user_agent_hash =
        crate::authorization_session_context_hash::authorization_session_context_hash(
            crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(&other_headers),
            first_peer,
        )
        .expect(constants_str::DIAGNOSTIC_90CE47EE);
    assert_ne!(
        same_context_hash.expose().as_ref(),
        other_user_agent_hash.expose().as_ref(),
    );
}
#[test]
fn test_audit_resource_identifier_uses_target_identifier() {
    assert_eq!(
        crate::admin_audit_resource_id::AdminAuditResourceId::User(
            server_admin_core::admin_user_record_id::AdminUserRecordId::try_from(42i64)
                .expect(constants_str::DIAGNOSTIC_423B91B9),
        )
        .value()
        .as_ref(),
        constants_str::VALUE_42
    );
    assert_eq!(
        crate::admin_audit_resource_id::AdminAuditResourceId::Role(
            server_admin_core::admin_role_record_id::AdminRoleRecordId::try_from(7i64)
                .expect(constants_str::DIAGNOSTIC_AF8DF9D2),
        )
        .value()
        .as_ref(),
        constants_str::VALUE_7902699B
    );
    assert_eq!(
        crate::admin_audit_resource_id::AdminAuditResourceId::SystemSettings
            .value()
            .as_ref(),
        constants_str::VALUE_1
    );
}
#[test]
fn test_open_api_contains_auth_and_user_security_contracts() {
    frontend_contract_validation::validate_openapi_schema_references::validate_openapi_schema_references(
        &utoipa::openapi::OpenApi::from(crate::admin_api_open_api::admin_api_open_api()),
    )
    .expect(constants_str::DIAGNOSTIC_2151641D);
    let document = serde_json::to_value(utoipa::openapi::OpenApi::from(
        crate::admin_api_open_api::admin_api_open_api(),
    ))
    .expect(constants_str::DIAGNOSTIC_869D28D7);
    let paths = document
        .get(constants_str::PATHS)
        .and_then(serde_json::Value::as_object)
        .expect(constants_str::DIAGNOSTIC_6E15EDEC);
    assert_eq!(paths.len(), 22usize);
    assert!(!paths.contains_key(constants_str::VALUE_F772F137));
    assert!(!paths.contains_key(constants_str::VALUE_1DFB120F));
    assert!(!paths.contains_key(constants_str::VALUE_D1688529));
    assert!(!paths.contains_key(constants_str::VALUE_69A70592));
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
                            .expect(constants_str::DIAGNOSTIC_4252ACC8)
                            .to_owned(),
                        path.to_owned(),
                    )
                })
        })
        .collect::<std::collections::BTreeSet<_>>();
    let contracted_route_contracts = <server_admin_contract::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::coverage_descriptors()
            .as_ref()
            .iter()
            .copied()
            .map(|descriptor| {
                let metadata = descriptor.get_metadata();
                (
                    metadata.method().as_ref().to_ascii_lowercase(),
                    metadata.openapi_operation_id().as_ref().to_owned(),
                    metadata.path().as_ref().to_owned(),
                )
            })
            .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(documented_route_contracts, contracted_route_contracts);
    assert!(paths.contains_key(constants_str::VALUE_C764A505));
    assert!(paths.contains_key(constants_str::VALUE_356A53CE));
    assert!(paths.contains_key(constants_str::VALUE_2A3105E4));
    assert!(paths.contains_key(constants_str::VALUE_FD625302));
    assert!(paths.contains_key(constants_str::VALUE_4690F648));
    assert!(paths.contains_key(constants_str::VALUE_FF2134BE));
    assert!(paths.contains_key(constants_str::VALUE_E40BCD1D));
    assert_eq!(
        document
            .pointer(constants_str::ADMIN_OPENAPI_SIGN_IN_OPERATION_ID_POINTER)
            .and_then(serde_json::Value::as_str),
        Some(
            <server_admin_contract::admin_sign_in_route::AdminSignInRoute as frontend_contract::typed_route::TypedRoute>::metadata()
                .openapi_operation_id()
                .as_ref()
        ),
    );
    assert_eq!(
        document
            .pointer(constants_str::ADMIN_OPENAPI_REFRESH_OPERATION_ID_POINTER)
            .and_then(serde_json::Value::as_str),
        Some(
            <server_admin_contract::admin_refresh_route::AdminRefreshRoute as frontend_contract::typed_route::TypedRoute>::metadata()
                .openapi_operation_id()
                .as_ref()
        ),
    );
    assert_eq!(
        document
            .pointer(constants_str::ADMIN_OPENAPI_ME_OPERATION_ID_POINTER)
            .and_then(serde_json::Value::as_str),
        Some(
            <server_admin_contract::admin_me_route::AdminMeRoute as frontend_contract::typed_route::TypedRoute>::metadata()
                .openapi_operation_id()
                .as_ref()
        ),
    );
    assert!(
        paths
            .values()
            .all(|path| path.as_object().is_some_and(|operations| operations
                .values()
                .all(|operation| operation.pointer(constants_str::VALUE_7BD7C79B).is_some())))
    );
    assert!(document.pointer(constants_str::VALUE_5223FAE7).is_some());
    assert!(document.pointer(constants_str::VALUE_03C3BB69).is_some());
    assert_eq!(
        document
            .pointer(constants_str::VALUE_6A277FCB)
            .and_then(serde_json::Value::as_bool),
        Some(true),
    );
    let expected_body_limit_description = format!(
            "{}{}",
            constants_str::OPENAPI_REQUEST_BODY_MAXIMUM_BYTES_PREFIX,
            <server_admin_contract::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::body_limit()
                .expect(constants_str::DIAGNOSTIC_BE105D90)
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

// Root-owned module compatibility wrappers.
