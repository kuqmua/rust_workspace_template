#[tokio::test]
async fn test_admin_error_response_statuses_preserve_public_bodies_with_internal_diagnostics() {
    let cases = [
        (
            frontend_contract::route_error_status::RouteErrorStatus::Authentication,
            http::StatusCode::UNAUTHORIZED,
            frontend_contract::api_problem_kind::ApiProblemKind::Authentication,
            constants_str::AUTHENTICATION_REQUIRED,
        ),
        (
            frontend_contract::route_error_status::RouteErrorStatus::Authorization,
            http::StatusCode::FORBIDDEN,
            frontend_contract::api_problem_kind::ApiProblemKind::Authorization,
            constants_str::AUTHORIZATION_FAILED,
        ),
        (
            frontend_contract::route_error_status::RouteErrorStatus::Conflict,
            http::StatusCode::CONFLICT,
            frontend_contract::api_problem_kind::ApiProblemKind::Conflict,
            constants_str::RESOURCE_STATE_CONFLICT,
        ),
        (
            frontend_contract::route_error_status::RouteErrorStatus::Internal,
            http::StatusCode::INTERNAL_SERVER_ERROR,
            frontend_contract::api_problem_kind::ApiProblemKind::Internal,
            constants_str::INTERNAL_SERVER_ERROR,
        ),
        (
            frontend_contract::route_error_status::RouteErrorStatus::MethodNotAllowed,
            http::StatusCode::METHOD_NOT_ALLOWED,
            frontend_contract::api_problem_kind::ApiProblemKind::MethodNotAllowed,
            constants_str::METHOD_NOT_ALLOWED,
        ),
        (
            frontend_contract::route_error_status::RouteErrorStatus::PayloadTooLarge,
            http::StatusCode::PAYLOAD_TOO_LARGE,
            frontend_contract::api_problem_kind::ApiProblemKind::PayloadTooLarge,
            constants_str::REQUEST_BODY_IS_TOO_LARGE,
        ),
        (
            frontend_contract::route_error_status::RouteErrorStatus::RateLimited,
            http::StatusCode::TOO_MANY_REQUESTS,
            frontend_contract::api_problem_kind::ApiProblemKind::RateLimited,
            constants_str::REQUEST_RATE_LIMIT_EXCEEDED_ALT,
        ),
        (
            frontend_contract::route_error_status::RouteErrorStatus::ServiceUnavailable,
            http::StatusCode::SERVICE_UNAVAILABLE,
            frontend_contract::api_problem_kind::ApiProblemKind::Internal,
            constants_str::INTERNAL_SERVER_ERROR,
        ),
        (
            frontend_contract::route_error_status::RouteErrorStatus::Validation,
            http::StatusCode::UNPROCESSABLE_ENTITY,
            frontend_contract::api_problem_kind::ApiProblemKind::Validation,
            constants_str::REQUEST_VALIDATION_FAILED,
        ),
    ];
    futures::stream::StreamExt::fold(
        futures::stream::iter(cases),
        (),
        async |(), (route_error_status, expected_status, expected_kind, expected_detail)| {
            let observed_error = server_observability::observed_error::ObservedError::capture(
                crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
                server_observability::observed_error_code::ObservedErrorCode::from(
                    constants_str::ADMIN_OBSERVED_ERROR_DATABASE,
                ),
            );
            let diagnostic =
                server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic::from_observed(
                    server_runtime_http::http_error_type::HttpErrorType::from(
                        constants_str::ADMIN_API_ERROR_TYPE,
                    ),
                    &observed_error,
                );
            let plain_response = crate::admin_error_response_parts::admin_error_response_parts(
                route_error_status,
                None,
            );
            let observed_response = crate::admin_error_response_parts::admin_error_response_parts(
                route_error_status,
                Some(diagnostic),
            );
            assert!(
                [&plain_response, &observed_response]
                    .into_iter()
                    .all(|response| {
                        response.status() == expected_status
                            && response.headers().get(http::header::CONTENT_TYPE)
                                == Some(&http::HeaderValue::from_static(
                                    constants_str::APPLICATION_PROBLEM_PLUS_JSON,
                                ))
                            && response.headers().get(http::header::RETRY_AFTER)
                                == (expected_status == http::StatusCode::TOO_MANY_REQUESTS)
                                    .then(|| {
                                        http::HeaderValue::from_static(constants_str::VALUE_60)
                                    })
                                    .as_ref()
                    })
            );
            assert!(
                plain_response
                    .extensions()
                    .get::<server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic>()
                    .is_none()
            );
            assert!(
                observed_response
                    .extensions()
                    .get::<server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic>()
                    .is_some_and(|attached| {
                        attached.telemetry().error_code().to_string()
                            == observed_error.error_code().get()
                            && attached.location().line() == observed_error.location().line()
                            && attached.location().file() == observed_error.location().file()
                    })
            );
            let (plain_bytes_result, observed_bytes_result) = futures::join!(
                axum::body::to_bytes(plain_response.into_body(), constants_usize::VALUE_1_048_576),
                axum::body::to_bytes(
                    observed_response.into_body(),
                    constants_usize::VALUE_1_048_576
                ),
            );
            assert!(plain_bytes_result.is_ok());
            assert!(observed_bytes_result.is_ok());
            let (Ok(plain_bytes), Ok(observed_bytes)) = (plain_bytes_result, observed_bytes_result)
            else {
                return;
            };
            assert_eq!(plain_bytes, observed_bytes);
            assert!(
                serde_json::from_slice::<frontend_contract::api_problem::ApiProblem>(
                    &observed_bytes
                )
                .is_ok_and(|problem| {
                    u16::from(problem.status()) == expected_status.as_u16()
                        && problem.kind() == expected_kind
                        && problem.detail().as_ref() == expected_detail
                        && problem.request_id().is_none()
                })
            );
        },
    )
    .await;
}

#[tokio::test]
async fn test_created_json_response_preserves_created_status_content_type_and_typed_body() {
    let admin_user_id = server_admin_contract::admin_user_id::AdminUserId::from(
        server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64::from(
            std::num::NonZeroI64::from(std::num::NonZeroU8::MIN),
        ),
    );
    let response = axum::response::IntoResponse::into_response(
        crate::created_json_response::created_json_response(admin_user_id),
    );
    assert_eq!(response.status(), http::StatusCode::CREATED);
    assert_eq!(
        response
            .headers()
            .get(http::header::CONTENT_TYPE)
            .and_then(|header| header.to_str().ok()),
        Some(constants_str::APPLICATION_JSON),
    );
    assert!(
        axum::body::to_bytes(response.into_body(), 16usize)
            .await
            .is_ok_and(|body| {
                serde_json::from_slice::<server_admin_contract::admin_user_id::AdminUserId>(&body)
                    .is_ok_and(|decoded| decoded == admin_user_id)
            })
    );
}

#[test]
fn test_unique_violation_mapping_preserves_non_conflict_database_sources_and_diagnostics() {
    assert!(
        [
            sqlx::Error::RowNotFound,
            sqlx::Error::PoolClosed,
            sqlx::Error::PoolTimedOut,
            sqlx::Error::Protocol(constants_str::X.to_owned()),
            sqlx::Error::Io(std::io::Error::from(std::io::ErrorKind::BrokenPipe)),
            sqlx::Error::Decode(Box::new(std::io::Error::from(
                std::io::ErrorKind::InvalidData
            ))),
        ]
        .into_iter()
        .all(|error| {
            let original = error.to_string();
            let mapped = crate::map_unique_violation::map_unique_violation(
                crate::sqlx_admin_error::SqlxAdminError::from(error).into_inner(),
            );
            let crate::admin_error::AdminError::Pg(observed) = &mapped else {
                return false;
            };
            observed.source_ref().get_inner().to_string() == original
                && observed.error_code()
                    == server_observability::observed_error_code::ObservedErrorCode::from(
                        constants_str::ADMIN_OBSERVED_ERROR_DATABASE,
                    )
                && std::error::Error::source(&mapped).is_some_and(|source| {
                    source.is::<server_observability::observed_error::ObservedError<
                        crate::sqlx_admin_error::SqlxAdminError,
                    >>() && source.to_string() == original
                })
                && std::error::Error::source(observed).is_some_and(|source| {
                    source.is::<crate::sqlx_admin_error::SqlxAdminError>()
                        && source.to_string() == original
                })
        })
    );
}
