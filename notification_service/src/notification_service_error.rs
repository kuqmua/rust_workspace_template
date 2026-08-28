#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum NotificationServiceError {
    #[error("notification service configuration failed: {0}")]
    Config(notification_service_config::config::ConfigTryFromEnvError),
    #[error("notification database connection failed: {0}")]
    Database(super::SqlxNotificationDatabaseError),
    #[error("notification metrics recorder initialization failed: {0}")]
    Metrics(super::MetricsExporterPrometheusNotificationBuildError),
    #[error("notification observability initialization failed: {0}")]
    ObservabilityInit(server_runtime_http::domain_types::ObservabilityInitError),
    #[error("notification observability shutdown failed: {0}")]
    ObservabilityShutdown(
        server_runtime_http::domain_types::OpentelemetrySdkObservabilityShutdownError,
    ),
    #[error("notification database migration failed: {0}")]
    Migration(super::SqlxNotificationMigrationError),
    #[error("notification service failed: {0}")]
    Serve(server_runtime_http::domain_types::ServeWithGracefulShutdownError),
    #[error("notification service socket bind failed: {0}")]
    Socket(super::NotificationIoError),
    #[error("notification service timeout configuration is invalid")]
    Timeout,
}
