#[track_caller]
pub(crate) fn admin_observed_internal_error_response<Source>(
    source: Source,
    observed_error_code: server_observability::observed_error_code::ObservedErrorCode,
) -> axum::response::Response
where
    Source: std::error::Error + 'static,
{
    let observed_error =
        server_observability::observed_error::ObservedError::capture(source, observed_error_code);
    let diagnostic = server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic::from_observed(
        server_runtime_http::http_error_type::HttpErrorType::from(
            constants_str::ADMIN_API_ERROR_TYPE,
        ),
        &observed_error,
    );
    crate::admin_error_response_parts::admin_error_response_parts(
        frontend_contract::route_error_status::RouteErrorStatus::Internal,
        Some(diagnostic),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_internal_response_preserves_error_code_and_call_site() {
        let expected_line = line!() + 2u32;
        let response =
            crate::admin_observed_internal_error_response::admin_observed_internal_error_response(
                crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
                server_observability::observed_error_code::ObservedErrorCode::from(
                    constants_str::ADMIN_OBSERVED_ERROR_DATABASE,
                ),
            );
        assert_eq!(response.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
        let diagnostic = response
            .extensions()
            .get::<server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic>();
        assert!(diagnostic.is_some());
        if let Some(diagnostic) = diagnostic {
            assert_eq!(diagnostic.location().line(), expected_line);
            assert_eq!(
                diagnostic.telemetry().error_type().to_string(),
                constants_str::ADMIN_API_ERROR_TYPE
            );
            assert_eq!(
                diagnostic.telemetry().error_code().to_string(),
                constants_str::ADMIN_OBSERVED_ERROR_DATABASE
            );
        }
    }
}
