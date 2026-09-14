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
