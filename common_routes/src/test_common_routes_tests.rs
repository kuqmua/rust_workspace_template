#[test]
fn test_uri_extractor_preserves_complete_uri_and_request_parts() {
    let check_uri = |uri: axum::http::Uri| {
        let mut request = axum::http::Request::new(());
        *request.uri_mut() = uri.clone();
        *request.method_mut() = axum::http::Method::POST;
        *request.version_mut() = axum::http::Version::HTTP_2;
        let _previous_header = request.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static(constants_str::APPLICATION_JSON),
        );
        let _previous_extension = request
            .extensions_mut()
            .insert(crate::health_database_available::HealthDatabaseAvailable::from(true));
        let (mut parts, ()) = request.into_parts();
        let result = {
            let mut future = std::pin::pin!(
            <crate::axum_http_uri::AxumHttpUri as axum::extract::FromRequestParts<()>>::from_request_parts(&mut parts, &())
        );
            let mut context = std::task::Context::from_waker(std::task::Waker::noop());
            Future::poll(future.as_mut(), &mut context)
        };
        assert!(
            matches!(&result, std::task::Poll::Ready(Ok(extracted)) if extracted.get() == &uri)
        );
        assert_eq!(parts.uri, uri);
        assert_eq!(parts.method, axum::http::Method::POST);
        assert_eq!(parts.version, axum::http::Version::HTTP_2);
        assert_eq!(parts.headers.len(), constants_usize::ONE);
        assert_eq!(
            parts.headers.get(axum::http::header::CONTENT_TYPE),
            Some(&axum::http::HeaderValue::from_static(
                constants_str::APPLICATION_JSON
            )),
        );
        assert!(
            parts
                .extensions
                .get::<crate::health_database_available::HealthDatabaseAvailable>()
                .is_some_and(|available| available.is_available())
        );
    };
    check_uri(axum::http::Uri::from_static(
        constants_str::MISSING_PATH_QUESTION_LIMIT_10,
    ));
    check_uri(axum::http::Uri::from_static(
        constants_str::HTTPS_ADMIN_EXAMPLE_COM_PATH,
    ));
}

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
async fn test_healthy_reports_and_liveness_handler_preserve_json_response_contract() {
    let check_response =
        async |json_response: crate::json_response::JsonResponse<
            crate::health_report::HealthReport,
        >,
               health_report: crate::health_report::HealthReport| {
            let response = axum::response::IntoResponse::into_response(json_response);
            assert_eq!(response.status(), axum::http::StatusCode::OK);
            assert_eq!(
                response
                    .headers()
                    .get(axum::http::header::CONTENT_TYPE)
                    .and_then(|value| value.to_str().ok()),
                Some(constants_str::APPLICATION_JSON)
            );
            let body =
                axum::body::to_bytes(response.into_body(), constants_usize::VALUE_1_048_576).await;
            assert!(body.is_ok_and(|body_bytes| {
                serde_json::from_slice::<crate::health_report::HealthReport>(&body_bytes)
                    .is_ok_and(|report| report == health_report)
            }));
        };
    let live = crate::health_report::HealthReport::liveness();
    let ready = crate::health_report::HealthReport::readiness(
        crate::health_database_available::HealthDatabaseAvailable::from(true),
    );
    let mapped_live = crate::health_report_response::health_report_response(live.clone());
    let mapped_ready = crate::health_report_response::health_report_response(ready.clone());
    assert!(mapped_live.is_ok());
    assert!(mapped_ready.is_ok());
    let (Ok(live_response), Ok(ready_response)) = (mapped_live, mapped_ready) else {
        return;
    };
    let handler_response = crate::health_live::health_live().await;
    let _checked = tokio::join!(
        check_response(live_response, live.clone()),
        check_response(ready_response, ready),
        check_response(handler_response, live)
    );
}

#[tokio::test]
async fn test_degraded_health_response_matches_route_contract() {
    let degraded_report = crate::health_report::HealthReport::readiness(
        crate::health_database_available::HealthDatabaseAvailable::from(false),
    );
    let encoded = serde_json::to_value(&degraded_report);
    assert!(encoded.is_ok());
    let Ok(mut encoded_report) = encoded else {
        return;
    };
    let status = encoded_report.get_mut(stringify!(status));
    assert!(status.is_some());
    let Some(status_value) = status else {
        return;
    };
    *status_value = serde_json::Value::String(String::from(stringify!(error)));
    let decoded = serde_json::from_value::<crate::health_report::HealthReport>(encoded_report);
    assert!(decoded.is_ok());
    let Ok(error_report) = decoded else {
        return;
    };
    assert_eq!(
        error_report.status(),
        crate::health_status::HealthStatus::Error
    );
    let check_unavailable = async |health_report: crate::health_report::HealthReport| {
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
        let body =
            axum::body::to_bytes(response.into_body(), constants_usize::VALUE_1_048_576).await;
        assert!(body.is_ok());
        let Ok(body_bytes) = body else {
            return;
        };
        assert!(matches!(
            serde_json::from_slice::<crate::health_report::HealthReport>(&body_bytes),
            Ok(report) if report == health_report
        ));
    };
    let _checked = tokio::join!(
        check_unavailable(degraded_report),
        check_unavailable(error_report)
    );
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

#[tokio::test]
async fn test_commit_link_errors_return_internal_http_status() {
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
    let expected_response = axum::response::IntoResponse::into_response(
        frontend_contract::api_problem_error::ApiProblemError::Internal(
            frontend_contract::api_problem_status::ApiProblemStatus::from(
                frontend_contract::known_http_status::KnownHttpStatus::InternalServerError,
            ),
        ),
    );
    assert_eq!(git_response.headers(), expected_response.headers());
    assert_eq!(fallback_response.headers(), expected_response.headers());
    let (git_body, fallback_body, expected_body) = tokio::join!(
        axum::body::to_bytes(git_response.into_body(), constants_usize::VALUE_1_048_576),
        axum::body::to_bytes(
            fallback_response.into_body(),
            constants_usize::VALUE_1_048_576
        ),
        axum::body::to_bytes(
            expected_response.into_body(),
            constants_usize::VALUE_1_048_576
        ),
    );
    assert!(git_body.is_ok() && fallback_body.is_ok() && expected_body.is_ok());
    let (Ok(git_bytes), Ok(fallback_bytes), Ok(expected_bytes)) =
        (git_body, fallback_body, expected_body)
    else {
        return;
    };
    assert_eq!(git_bytes, expected_bytes);
    assert_eq!(fallback_bytes, expected_bytes);
    assert!(
        <crate::git_info_route::GitInfoRoute as frontend_contract::typed_route::TypedRoute>::metadata()
            .error_statuses()
            .contains(&frontend_contract::route_error_status::RouteErrorStatus::Internal)
    );
}
