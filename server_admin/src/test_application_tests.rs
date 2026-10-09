#[test]
fn test_rate_limit_scopes_are_distinct() {
    let scopes = [
        crate::admin_rate_limit_scope::AdminRateLimitScope::Mutation,
        crate::admin_rate_limit_scope::AdminRateLimitScope::RefreshIp,
        crate::admin_rate_limit_scope::AdminRateLimitScope::SignInIp,
        crate::admin_rate_limit_scope::AdminRateLimitScope::SignInIpLogin,
    ]
    .map(crate::admin_rate_limit_scope::AdminRateLimitScope::as_str);
    let unique = scopes.into_iter().collect::<std::collections::HashSet<_>>();
    assert_eq!(unique.len(), 4usize);
}
#[tokio::test]
async fn test_create_user_payload_example_matches_create_request_contract() {
    let result = crate::api_create_user_payload_example::api_create_user_payload_example().await;
    assert!(result.is_ok());
    if let Ok(axum_admin_response) = result {
        let response = axum::response::Response::from(axum_admin_response);
        assert_eq!(response.status(), http::StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), 16_384usize).await;
        assert!(body.is_ok());
        if let Ok(bytes) = body {
            let payload = serde_json::from_slice::<
                server_admin_contract::admin_create_users_request::AdminCreateUsersRequest,
            >(bytes.as_ref());
            assert!(payload.is_ok());
            if let Ok(requests) = payload {
                let requests = AsRef::<
                    [server_admin_contract::admin_create_user_request::AdminCreateUserRequest],
                >::as_ref(&requests);
                assert_eq!(requests.len(), 1usize);
                if let Some(request) = requests.first().cloned() {
                    let (display_name, login, password, role_ids) = request.into_parts();
                    assert_eq!(
                        display_name.as_ref(),
                        constants_str::ADMIN_FIXTURE_ALPHA_DISPLAY_NAME
                    );
                    assert_eq!(login.as_ref(), constants_str::ADMIN_FIXTURE_ALPHA_LOGIN);
                    assert_eq!(password.as_ref(), constants_str::TEST_STRONG_PASSWORD);
                    assert!(role_ids.is_none());
                }
            }
        }
    }
}
#[tokio::test]
async fn test_delete_users_payload_example_matches_delete_request_contract() {
    let result = crate::api_delete_users_payload_example::api_delete_users_payload_example().await;
    assert!(result.is_ok());
    if let Ok(axum_admin_response) = result {
        let response = axum::response::Response::from(axum_admin_response);
        assert_eq!(response.status(), http::StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), 16_384usize).await;
        assert!(body.is_ok());
        if let Ok(bytes) = body {
            let payload = serde_json::from_slice::<
                server_admin_contract::admin_delete_users_request::AdminDeleteUsersRequest,
            >(bytes.as_ref());
            assert!(payload.is_ok_and(|request| {
                request
                    .filter()
                    .user_id()
                    .copied()
                    .is_some_and(|user_id| i64::from(user_id) == constants_i64::ONE)
            }));
        }
    }
}
#[tokio::test]
async fn test_role_payload_examples_match_role_mutation_request_contracts() {
    let create_result =
        crate::api_create_roles_payload_example::api_create_roles_payload_example().await;
    let update_result =
        crate::api_update_roles_payload_example::api_update_roles_payload_example().await;
    let delete_result =
        crate::api_delete_roles_payload_example::api_delete_roles_payload_example().await;
    assert!(create_result.is_ok());
    assert!(update_result.is_ok());
    assert!(delete_result.is_ok());
    if let (Ok(create), Ok(update), Ok(delete)) = (create_result, update_result, delete_result) {
        let create_body = axum::body::to_bytes(
            axum::response::Response::from(create).into_body(),
            16_384usize,
        )
        .await;
        let update_body = axum::body::to_bytes(
            axum::response::Response::from(update).into_body(),
            16_384usize,
        )
        .await;
        let delete_body = axum::body::to_bytes(
            axum::response::Response::from(delete).into_body(),
            16_384usize,
        )
        .await;
        assert!(create_body.is_ok_and(|bytes| {
            serde_json::from_slice::<
                server_admin_contract::admin_create_roles_request::AdminCreateRolesRequest,
            >(bytes.as_ref())
            .is_ok_and(|request| request.as_ref().len() == 1usize)
        }));
        assert!(update_body.is_ok_and(|bytes| {
            serde_json::from_slice::<
                server_admin_contract::admin_update_roles_request::AdminUpdateRolesRequest,
            >(bytes.as_ref())
            .is_ok_and(|request| request.updates().as_ref().len() == 1usize)
        }));
        assert!(delete_body.is_ok_and(|bytes| {
            serde_json::from_slice::<
                server_admin_contract::admin_delete_roles_request::AdminDeleteRolesRequest,
            >(bytes.as_ref())
            .is_ok_and(|request| request.filter().get_role_id().is_some())
        }));
    }
}
#[tokio::test]
async fn test_update_users_payload_example_matches_update_request_contract() {
    let result = crate::api_update_users_payload_example::api_update_users_payload_example().await;
    assert!(result.is_ok());
    if let Ok(axum_admin_response) = result {
        let response = axum::response::Response::from(axum_admin_response);
        assert_eq!(response.status(), http::StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), 16_384usize).await;
        assert!(body.is_ok());
        if let Ok(bytes) = body {
            let payload = serde_json::from_slice::<
                server_admin_contract::admin_update_users_request::AdminUpdateUsersRequest,
            >(bytes.as_ref());
            assert!(payload.is_ok_and(|request| {
                request.updates().as_ref().first().is_some_and(|update| {
                    update
                        .filter()
                        .user_id()
                        .copied()
                        .is_some_and(|user_id| i64::from(user_id) == constants_i64::ONE)
                        && update
                            .changes()
                            .is_banned()
                            .copied()
                            .is_some_and(|is_banned| !bool::from(is_banned))
                }) && request.updates().as_ref().len() == 1usize
            }));
        }
    }
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
    assert_eq!(
        same_context_hash.expose().as_ref(),
        base16ct::lower::encode_string(&[
            0x35u8, 0xc1, 0xa3, 0xcf, 0x97, 0xd0, 0x95, 0x1e, 0x28, 0x64, 0x6d, 0x73, 0x8d, 0x43,
            0xcb, 0x69, 0xb7, 0x4c, 0xcf, 0x28, 0xbe, 0xcb, 0xa9, 0x76, 0x2e, 0x41, 0xe5, 0x8a,
            0x8e, 0x46, 0x69, 0x6c,
        ]),
    );
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
    assert_eq!(
        other_peer_hash.expose().as_ref(),
        base16ct::lower::encode_string(&[
            0x41u8, 0x15, 0x4b, 0x18, 0x12, 0x60, 0x1d, 0xfa, 0x34, 0xd5, 0x86, 0x06, 0x38, 0xa5,
            0x2a, 0x01, 0x50, 0x72, 0xe1, 0x3d, 0x8e, 0xe8, 0x04, 0x9a, 0x48, 0x8b, 0x00, 0x3b,
            0xf5, 0x95, 0x94, 0x1f,
        ]),
    );
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
    assert_eq!(
        other_user_agent_hash.expose().as_ref(),
        base16ct::lower::encode_string(&[
            0x29u8, 0xe5, 0x6c, 0x60, 0xe4, 0xa9, 0xb4, 0x4b, 0xba, 0x02, 0x55, 0xd7, 0x2c, 0x07,
            0x57, 0x4e, 0xcc, 0x64, 0x9f, 0xe9, 0x06, 0xd2, 0xa0, 0x29, 0x57, 0x14, 0xdb, 0x3c,
            0xfa, 0x35, 0xec, 0xcc,
        ]),
    );
    assert_ne!(
        same_context_hash.expose().as_ref(),
        other_user_agent_hash.expose().as_ref(),
    );
}
#[test]
fn test_audit_resource_identifier_uses_target_identifier() {
    [1i64, 17i64, i64::MAX].into_iter().for_each(|value| {
        assert!(
            crate::admin_session_id::AdminSessionId::try_from(value).is_ok_and(
                |admin_session_id| {
                    let identifier = crate::admin_audit_resource_id::AdminAuditResourceId::Session(
                        admin_session_id,
                    )
                    .value();
                    identifier.as_ref().as_str() == value.to_string()
                        && identifier.as_ref().parse::<i64>() == Ok(value)
                }
            )
        );
    });
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
    assert_eq!(paths.len(), 30usize);
    assert!(paths.contains_key(constants_str::ADMIN_ROLES_CREATE_PAYLOAD_EXAMPLE_READ));
    assert!(paths.contains_key(constants_str::ADMIN_ROLES_UPDATE_PAYLOAD_EXAMPLE_READ));
    assert!(paths.contains_key(constants_str::ADMIN_ROLES_DELETE_PAYLOAD_EXAMPLE_READ));
    assert!(paths.contains_key(constants_str::ADMIN_USERS_CREATE_PAYLOAD_EXAMPLE_READ));
    assert!(paths.contains_key(constants_str::ADMIN_USERS_DELETE_PAYLOAD_EXAMPLE_READ));
    assert!(paths.contains_key(constants_str::ADMIN_USERS_UPDATE_PAYLOAD_EXAMPLE_READ));
    assert!(!paths.contains_key(constants_str::VALUE_2C49C991));
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
                if matches!(
                    metadata.route_method(),
                    frontend_contract::route_method::RouteMethod::Get
                        | frontend_contract::route_method::RouteMethod::Post
                ) && metadata.mutation()
                    == frontend_contract::route_mutation::RouteMutation::ReadOnly
                {
                    assert!(
                        metadata
                            .path()
                            .as_ref()
                            .ends_with(constants_str::READ_ROUTE_SUFFIX)
                    );
                }
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
    assert!(!paths.contains_key(constants_str::VALUE_2A3105E4));
    assert!(!paths.contains_key(constants_str::VALUE_4690F648));
    assert!(!paths.contains_key(constants_str::VALUE_FF2134BE));
    assert!(!paths.contains_key(constants_str::VALUE_E40BCD1D));
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

#[test]
fn test_admin_openapi_security_schemes_preserve_wire_contracts() {
    let document = utoipa::openapi::OpenApi::from(crate::admin_api_open_api::admin_api_open_api());
    assert!(document.components.as_ref().is_some_and(|components| {
        [
            (constants_str::ADMIN_COOKIE, serde_json::json!({
                (stringify!(type)): stringify!(apiKey),
                (stringify!(in)): stringify!(cookie),
                (stringify!(name)): constants_str::SERVER_ADMIN_ACCESS_COOKIE_NAME,
                (stringify!(description)): constants_str::HTTPONLY_ADMINISTRATOR_ACCESS_TOKEN_COOKIE,
            })),
            (constants_str::ADMIN_CSRF, serde_json::json!({
                (stringify!(type)): stringify!(apiKey),
                (stringify!(in)): stringify!(header),
                (stringify!(name)): constants_str::X_CSRF_TOKEN,
                (stringify!(description)): constants_str::CSRF_TOKEN_BOUND_TO_THE_ADMINISTRATOR_ACCESS_SESSION,
            })),
        ].into_iter().all(|(name, expected)| {
            components.security_schemes.get(name).is_some_and(|scheme| {
                serde_json::to_value(scheme).is_ok_and(|actual| actual == expected)
            })
        })
    }));
}
#[tokio::test]
async fn test_admin_mutation_payload_examples_preserve_exact_wire_contracts() {
    async fn test_assert_payload_example_wire_response<Error>(
        result: Result<crate::axum_admin_response::AxumAdminResponse, Error>,
        std_admin_string_result: Result<
            server_admin_core::std_admin_string::StdAdminString,
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError,
        >,
    ) where
        Error: std::error::Error,
    {
        assert!(result.is_ok());
        assert!(std_admin_string_result.is_ok());
        if let (Ok(axum_admin_response), Ok(std_admin_string)) = (result, std_admin_string_result) {
            let response = axum::response::Response::from(axum_admin_response);
            assert_eq!(response.status(), http::StatusCode::OK);
            assert_eq!(
                response.headers().get(http::header::CONTENT_TYPE),
                Some(&http::HeaderValue::from_static(
                    constants_str::APPLICATION_JSON
                ))
            );
            let body = axum::body::to_bytes(response.into_body(), 16_384usize).await;
            assert!(body.is_ok());
            if let Ok(bytes) = body {
                let value = serde_json::from_slice::<serde_json::Value>(bytes.as_ref());
                let expected_json_value =
                    serde_json::from_str::<serde_json::Value>(std_admin_string.as_ref().as_str());
                assert!(value.is_ok());
                assert!(expected_json_value.is_ok());
                if let (Ok(value), Ok(expected_json_value)) = (value, expected_json_value) {
                    assert_eq!(value, expected_json_value);
                }
            }
        }
    }
    test_assert_payload_example_wire_response(
        crate::api_create_roles_payload_example::api_create_roles_payload_example().await,
        server_admin_core::std_admin_string::StdAdminString::try_from(
            serde_json::json!([
                {(stringify!(name)): constants_str::ADMIN_FIXTURE_ROLE_NAME}
            ])
            .to_string(),
        ),
    )
    .await;
    test_assert_payload_example_wire_response(
        crate::api_update_roles_payload_example::api_update_roles_payload_example().await,
        server_admin_core::std_admin_string::StdAdminString::try_from(
            serde_json::json!({
                (stringify!(updates)): [{
                    (stringify!(changes)): {
                        (stringify!(name)): constants_str::ADMIN_FIXTURE_ROLE_NAME,
                        (stringify!(rules)): null,
                    },
                    (stringify!(filter)): {
                        (stringify!(role_id)): 1i64,
                        (stringify!(name)): null,
                        (stringify!(is_system)): null,
                    },
                }],
            })
            .to_string(),
        ),
    )
    .await;
    test_assert_payload_example_wire_response(
        crate::api_delete_roles_payload_example::api_delete_roles_payload_example().await,
        server_admin_core::std_admin_string::StdAdminString::try_from(
            serde_json::json!({
                (stringify!(filter)): {
                    (stringify!(role_id)): 1i64,
                    (stringify!(name)): null,
                    (stringify!(is_system)): null,
                },
            })
            .to_string(),
        ),
    )
    .await;
    test_assert_payload_example_wire_response(
        crate::api_create_user_payload_example::api_create_user_payload_example().await,
        server_admin_core::std_admin_string::StdAdminString::try_from(
            serde_json::json!([{
                (stringify!(display_name)): constants_str::ADMIN_FIXTURE_ALPHA_DISPLAY_NAME,
                (stringify!(login)): constants_str::ADMIN_FIXTURE_ALPHA_LOGIN,
                (stringify!(password)): constants_str::TEST_STRONG_PASSWORD,
                (stringify!(role_ids)): null,
            }])
            .to_string(),
        ),
    )
    .await;
    test_assert_payload_example_wire_response(
        crate::api_update_users_payload_example::api_update_users_payload_example().await,
        server_admin_core::std_admin_string::StdAdminString::try_from(
            serde_json::json!({
                (stringify!(updates)): [{
                    (stringify!(changes)): {
                        (stringify!(display_name)): null,
                        (stringify!(login)): null,
                        (stringify!(password)): null,
                        (stringify!(expected_role_ids)): null,
                        (stringify!(role_ids)): null,
                        (stringify!(is_banned)): false,
                    },
                    (stringify!(filter)): {
                        (stringify!(user_id)): 1i64,
                        (stringify!(login)): null,
                        (stringify!(display_name)): null,
                        (stringify!(is_banned)): null,
                    },
                }],
            })
            .to_string(),
        ),
    )
    .await;
    test_assert_payload_example_wire_response(
        crate::api_delete_users_payload_example::api_delete_users_payload_example().await,
        server_admin_core::std_admin_string::StdAdminString::try_from(
            serde_json::json!({
                (stringify!(filter)): {
                    (stringify!(user_id)): 1i64,
                    (stringify!(login)): null,
                    (stringify!(display_name)): null,
                    (stringify!(is_banned)): null,
                },
            })
            .to_string(),
        ),
    )
    .await;
}

#[tokio::test]
async fn test_shared_method_router_dispatches_exact_contract_methods() {
    let accepted = futures::StreamExt::all(
        futures::stream::iter([
            (
                frontend_contract::route_method::RouteMethod::Connect,
                http::Method::CONNECT,
            ),
            (
                frontend_contract::route_method::RouteMethod::Delete,
                http::Method::DELETE,
            ),
            (
                frontend_contract::route_method::RouteMethod::Get,
                http::Method::GET,
            ),
            (
                frontend_contract::route_method::RouteMethod::Head,
                http::Method::HEAD,
            ),
            (
                frontend_contract::route_method::RouteMethod::Options,
                http::Method::OPTIONS,
            ),
            (
                frontend_contract::route_method::RouteMethod::Patch,
                http::Method::PATCH,
            ),
            (
                frontend_contract::route_method::RouteMethod::Post,
                http::Method::POST,
            ),
            (
                frontend_contract::route_method::RouteMethod::Put,
                http::Method::PUT,
            ),
            (
                frontend_contract::route_method::RouteMethod::Trace,
                http::Method::TRACE,
            ),
        ]),
        async |(route_method, method)| {
            let method_router = frontend_contract::route_method_router::route_method_router::<
                (),
                _,
                _,
            >(route_method, async || http::StatusCode::NO_CONTENT);
            let router = axum::Router::new().route(
                constants_str::SLASH,
                axum::routing::MethodRouter::from(method_router),
            );
            let request_result = http::Request::builder()
                .method(method)
                .uri(constants_str::SLASH)
                .body(axum::body::Body::empty());
            assert!(request_result.as_ref().err().is_none());
            if let Ok(request) = request_result {
                let response = tower::ServiceExt::oneshot(router.clone(), request).await;
                assert!(
                    response
                        .is_ok_and(|response| response.status() == http::StatusCode::NO_CONTENT)
                );
            }
            let rejected_method =
                if route_method == frontend_contract::route_method::RouteMethod::Post {
                    http::Method::GET
                } else {
                    http::Method::POST
                };
            let rejected_request_result = http::Request::builder()
                .method(rejected_method)
                .uri(constants_str::SLASH)
                .body(axum::body::Body::empty());
            assert!(rejected_request_result.as_ref().err().is_none());
            if let Ok(request) = rejected_request_result {
                tower::ServiceExt::oneshot(router, request)
                    .await
                    .is_ok_and(|response| response.status() == http::StatusCode::METHOD_NOT_ALLOWED)
            } else {
                false
            }
        },
    )
    .await;
    assert!(accepted);
}
