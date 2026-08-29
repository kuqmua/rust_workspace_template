#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum MetricsError {
    #[error("notification metrics response rendering failed: {0}")]
    Render(
        #[source]
        server_observability::observed_error::ObservedError<
            server_runtime_http::metrics_response_body_error::MetricsResponseBodyError,
        >,
    ),
}
impl axum::response::IntoResponse for MetricsError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Render(error) => {
                let error_type = server_runtime_http::http_error_type::HttpErrorType::from(
                    constants_str::test_fixtures::NOTIFICATION_API_ERROR_TYPE,
                );
                let mut response = axum::response::IntoResponse::into_response(
                    frontend_contract::api_problem_error::ApiProblemError::Internal(
                        frontend_contract::api_problem_status::ApiProblemStatus::from(
                            frontend_contract::known_http_status::KnownHttpStatus::InternalServerError,
                        ),
                    ),
                );
                let _previous = response.extensions_mut().insert(
                    server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic::from_observed(
                        error_type, &error,
                    ),
                );
                response
            }
        }
    }
}
