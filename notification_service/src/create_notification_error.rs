#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum CreateNotificationError {
    #[error("notification persistence failed: {0}")]
    Persistence(
        #[source]
        server_observability::observed_error::ObservedError<
            crate::sqlx_notification_database_error::SqlxNotificationDatabaseError,
        >,
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
        let error_type = server_runtime_http::http_error_type::HttpErrorType::from(
            constants_str::NOTIFICATION_API_ERROR_TYPE,
        );
        let optional_diagnostic = match &self {
            Self::Persistence(error) => Some(
                server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic::from_observed(
                    error_type, error,
                ),
            ),
            Self::Validation => None,
        };
        let telemetry = server_runtime_http::http_error_telemetry::HttpErrorTelemetry::new(
            error_type,
            server_runtime_http::http_error_code::HttpErrorCode::from(
                crate::notification_error_code::NotificationErrorCode::Validation.get(),
            ),
        );
        let problem_status =
            frontend_contract::api_problem_status::ApiProblemStatus::try_from(status.as_u16())
                .unwrap_or_else(|_error| {
                    frontend_contract::api_problem_status::ApiProblemStatus::from(
                        frontend_contract::known_http_status::KnownHttpStatus::InternalServerError,
                    )
                });
        let mut response = axum::response::IntoResponse::into_response(
            frontend_contract::api_problem_error::ApiProblemError::from_status(problem_status),
        );
        if let Some(diagnostic) = optional_diagnostic {
            let _previous = response.extensions_mut().insert(diagnostic);
        } else {
            let _previous = response.extensions_mut().insert(telemetry);
        }
        response
    }
}
