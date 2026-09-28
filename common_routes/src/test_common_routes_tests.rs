#[test]
fn test_common_routes_tests() {
    assert!(matches!(
        crate::health_report_response::health_report_response(
            crate::health_report::HealthReport::liveness()
        ),
        Ok(_response)
    ));
    assert!(matches!(
        crate::health_report_response::health_report_response(
            crate::health_report::HealthReport::readiness(
                crate::health_database_available::HealthDatabaseAvailable::from(false),
            )
        ),
        Err(crate::health_error::HealthError::Unavailable(_report))
    ));
}

#[tokio::test]
async fn test_degraded_health_response_matches_route_contract() {
    let health_report = crate::health_report::HealthReport::readiness(
        crate::health_database_available::HealthDatabaseAvailable::from(false),
    );
    let result = crate::health_report_response::health_report_response(health_report.clone());
    assert!(result.is_err());
    let Err(error) = result else {
        return;
    };
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(
        response.status(),
        axum::http::StatusCode::SERVICE_UNAVAILABLE
    );
    let body = axum::body::to_bytes(response.into_body(), constants_usize::VALUE_1_048_576).await;
    assert!(body.is_ok());
    let Ok(body_bytes) = body else {
        return;
    };
    assert!(matches!(
        serde_json::from_slice::<crate::health_report::HealthReport>(&body_bytes),
        Ok(report) if report == health_report
    ));
}

#[tokio::test]
async fn test_unavailable_health_check_has_empty_response_body() {
    let response = axum::response::IntoResponse::into_response(
        crate::health_check_error::HealthCheckError::Unavailable,
    );
    assert_eq!(
        response.status(),
        axum::http::StatusCode::SERVICE_UNAVAILABLE
    );
    assert!(matches!(
        axum::body::to_bytes(response.into_body(), constants_usize::VALUE_1_048_576).await,
        Ok(body) if body.is_empty()
    ));
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
