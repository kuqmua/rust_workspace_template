#[test]
fn test_common_routes_tests() {
    assert!(
        crate::health_report_response::health_report_response(
            crate::health_report::HealthReport::liveness()
        )
        .is_some()
    );
    assert!(
        crate::health_report_response::health_report_response(
            crate::health_report::HealthReport::readiness(
                crate::health_database_available::HealthDatabaseAvailable::from(false),
            )
        )
        .is_none()
    );
}

#[test]
fn test_commit_link_errors_return_internal_http_status() {
    let error =
        git_info::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong {
            len: constants_usize::ONE,
            max: constants_usize::ZERO,
        };
    let git_response = axum::response::IntoResponse::into_response(
        crate::git_info_response_error::GitInfoResponseError::CommitLink(error),
    );
    let fallback_response = axum::response::IntoResponse::into_response(
        crate::common_not_found_error::CommonNotFoundError::CommitLink(error),
    );
    assert_eq!(
        git_response.status(),
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        fallback_response.status(),
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    );
    assert!(
        <crate::git_info_route::GitInfoRoute as frontend_contract::typed_route::TypedRoute>::metadata()
            .error_statuses()
            .contains(&frontend_contract::route_error_status::RouteErrorStatus::Internal)
    );
}
