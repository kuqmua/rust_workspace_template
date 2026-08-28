#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum MetricsError {
    #[error("notification metrics response rendering failed: {0}")]
    Render(
        #[source]
        server_runtime_http::domain_types::ObservedError<
            server_runtime_http::domain_types::MetricsResponseBodyError,
        >,
    ),
}
impl axum::response::IntoResponse for MetricsError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Render(error) => {
                let error_type = server_runtime_http::domain_types::HttpErrorType::from(
                    constants_str::NOTIFICATION_API_ERROR_TYPE,
                );
                let mut response = axum::response::IntoResponse::into_response(
                    frontend_contract::ApiProblemError::Internal(
                        frontend_contract::ApiProblemStatus::from(
                            frontend_contract::KnownHttpStatus::InternalServerError,
                        ),
                    ),
                );
                let _previous = response.extensions_mut().insert(
                    server_runtime_http::domain_types::HttpErrorDiagnostic::from_observed(
                        error_type, &error,
                    ),
                );
                response
            }
        }
    }
}
