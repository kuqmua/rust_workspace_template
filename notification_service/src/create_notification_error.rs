#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum CreateNotificationError {
    #[error("notification persistence failed: {0}")]
    Persistence(
        #[source]
        server_runtime_http::domain_types::ObservedError<super::SqlxNotificationDatabaseError>,
    ),
    #[error("notification request validation failed")]
    Validation,
}
impl axum::response::IntoResponse for CreateNotificationError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            Self::Persistence(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::Validation => http::StatusCode::UNPROCESSABLE_ENTITY,
        };
        let error_type = server_runtime_http::domain_types::HttpErrorType::from(
            constants_str::NOTIFICATION_API_ERROR_TYPE,
        );
        let optional_diagnostic = match &self {
            Self::Persistence(error) => Some(
                server_runtime_http::domain_types::HttpErrorDiagnostic::from_observed(
                    error_type, error,
                ),
            ),
            Self::Validation => None,
        };
        let telemetry = server_runtime_http::domain_types::HttpErrorTelemetry::new(
            error_type,
            server_runtime_http::domain_types::HttpErrorCode::from(
                super::NotificationErrorCode::Validation.get(),
            ),
        );
        let problem_status =
            frontend_contract::domain_types::ApiProblemStatus::try_from(status.as_u16())
                .unwrap_or_else(|_error| {
                    frontend_contract::domain_types::ApiProblemStatus::from(
                        frontend_contract::domain_types::KnownHttpStatus::InternalServerError,
                    )
                });
        let mut response = axum::response::IntoResponse::into_response(
            frontend_contract::domain_types::ApiProblemError::from_status(problem_status),
        );
        if let Some(diagnostic) = optional_diagnostic {
            let _previous = response.extensions_mut().insert(diagnostic);
        } else {
            let _previous = response.extensions_mut().insert(telemetry);
        }
        response
    }
}
