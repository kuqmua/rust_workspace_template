#![allow(clippy::single_call_fn)] // binary composition functions intentionally have one startup or route registration owner
#![allow(clippy::arbitrary_source_item_ordering)] // OpenAPI document stays next to its generated schema and operation marker
#![allow(clippy::needless_for_each)] // utoipa OpenApi derive expands to an internal for_each
#![allow(clippy::field_scoped_visibility_modifiers)] // sibling application and adapter modules consume these private binary domain models

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(crate) struct NotificationState {
    pub(crate) metrics: MetricsExporterPrometheusRenderer,
    pub(crate) pool: app_state::domain_types::SqlxPgPool,
    pub(crate) project_git_info: git_info::domain_types::ProjectGitInfo<'static>,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(crate) struct AxumNotificationState(NotificationState);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AxumNotificationJson(
    notification_service_contract::domain_types::CreateNotificationReq,
);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AxumNotificationResponse(axum::response::Response);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AxumNotificationRouter(axum::Router);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct HttpNotificationStatusCode(http::StatusCode);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum CreateNotificationError {
    #[error("notification persistence failed: {0}")]
    Persistence(
        #[source] server_runtime_http::domain_types::ObservedError<SqlxNotificationDatabaseError>,
    ),
    #[error("notification request validation failed")]
    Validation,
}
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(crate) struct MetricsExporterPrometheusRenderer(metrics_exporter_prometheus::PrometheusHandle);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct NotificationBodyMaximumBytes(usize);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct NotificationExitCode(std::process::ExitCode);

impl AxumNotificationState {
    pub(crate) const fn get(&self) -> &NotificationState {
        &self.0
    }
}
impl AxumNotificationJson {
    pub(crate) fn into_inner(
        self,
    ) -> notification_service_contract::domain_types::CreateNotificationReq {
        self.0
    }
}
impl AxumNotificationRouter {
    pub(crate) fn into_inner(self) -> axum::Router {
        self.0
    }
}
impl MetricsExporterPrometheusRenderer {
    pub(crate) fn render(
        &self,
    ) -> Result<
        server_runtime_http::domain_types::MetricsResponseBody,
        server_runtime_http::domain_types::MetricsResponseBodyError,
    > {
        server_runtime_http::domain_types::MetricsResponseBody::try_from(self.0.render())
    }
}
impl NotificationBodyMaximumBytes {
    pub(crate) const fn get(self) -> usize {
        self.0
    }
}

impl axum::response::IntoResponse for AxumNotificationResponse {
    fn into_response(self) -> axum::response::Response {
        self.0
    }
}
impl axum::response::IntoResponse for HttpNotificationStatusCode {
    fn into_response(self) -> axum::response::Response {
        axum::response::IntoResponse::into_response(self.0)
    }
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
                NotificationErrorCode::Validation.get(),
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
impl axum::response::IntoResponse for MetricsError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Render(error) => {
                let error_type = server_runtime_http::domain_types::HttpErrorType::from(
                    constants_str::NOTIFICATION_API_ERROR_TYPE,
                );
                let mut response = axum::response::IntoResponse::into_response(
                    frontend_contract::domain_types::ApiProblemError::Internal(
                        frontend_contract::domain_types::ApiProblemStatus::from(
                            frontend_contract::domain_types::KnownHttpStatus::InternalServerError,
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
impl std::process::Termination for NotificationExitCode {
    fn report(self) -> std::process::ExitCode {
        self.0
    }
}
impl axum::extract::FromRequestParts<NotificationState> for AxumNotificationState {
    type Rejection = HttpNotificationStatusCode;
    fn from_request_parts(
        _parts: &mut http::request::Parts,
        state: &NotificationState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(Self::from(state.clone())))
    }
}
impl app_state::domain_types::SqlxPgPoolProvider for NotificationState {
    fn sqlx_pg_pool(&self) -> app_state::domain_types::SqlxPgPoolRef<'_> {
        app_state::domain_types::SqlxPgPoolRef::from(self.pool.as_ref())
    }
}
impl AsRef<str> for NotificationState {
    fn as_ref(&self) -> &str {
        self.project_git_info.as_ref()
    }
}
impl common_routes::domain_types::CommonRoutesParameters for NotificationState {}
impl axum::extract::FromRequest<NotificationState> for AxumNotificationJson {
    type Rejection = CreateNotificationError;
    async fn from_request(
        req: axum::extract::Request,
        state: &NotificationState,
    ) -> Result<Self, Self::Rejection> {
        <axum::Json<notification_service_contract::domain_types::CreateNotificationReq> as axum::extract::FromRequest<NotificationState>>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self::from(value))
            .map_err(|_error| CreateNotificationError::Validation)
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum NotificationServiceError {
    #[error("notification service configuration failed: {0}")]
    Config(NotificationConfigError),
    #[error("notification database connection failed: {0}")]
    Database(SqlxNotificationDatabaseError),
    #[error("notification metrics recorder initialization failed: {0}")]
    Metrics(MetricsExporterPrometheusNotificationBuildError),
    #[error("notification observability initialization failed: {0}")]
    ObservabilityInit(NotificationObservabilityInitError),
    #[error("notification observability shutdown failed: {0}")]
    ObservabilityShutdown(NotificationObservabilityShutdownError),
    #[error("notification database migration failed: {0}")]
    Migration(SqlxNotificationMigrationError),
    #[error("notification service failed: {0}")]
    Serve(NotificationServeError),
    #[error("notification service socket bind failed: {0}")]
    Socket(NotificationIoError),
    #[error("notification service timeout configuration is invalid")]
    Timeout,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct NotificationConfigError(
    notification_service_config::config::ConfigTryFromEnvError,
);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct SqlxNotificationDatabaseError(sqlx::Error);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct SqlxNotificationMigrationError(sqlx::migrate::MigrateError);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct NotificationIoError(std::io::Error);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct NotificationServeError(
    server_runtime_http::domain_types::ServeWithGracefulShutdownError,
);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct MetricsExporterPrometheusNotificationBuildError(
    metrics_exporter_prometheus::BuildError,
);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct NotificationObservabilityInitError(
    server_runtime_http::domain_types::ObservabilityInitError,
);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct NotificationObservabilityShutdownError(
    server_runtime_http::domain_types::OpentelemetrySdkObservabilityShutdownError,
);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NotificationErrorCode {
    MetricsRender,
    Persistence,
    Validation,
}
impl NotificationErrorCode {
    pub(crate) const fn get(self) -> &'static str {
        match self {
            Self::MetricsRender => constants_str::NOTIFICATION_OBSERVED_ERROR_METRICS_RENDER,
            Self::Persistence => constants_str::NOTIFICATION_OBSERVED_ERROR_PERSISTENCE,
            Self::Validation => constants_str::NOTIFICATION_OBSERVED_ERROR_VALIDATION,
        }
    }
}
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
