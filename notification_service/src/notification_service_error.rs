#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum NotificationServiceError {
    #[error("notification service configuration failed: {0}")]
    Config(notification_service_config::config::ConfigTryFromEnvError),
    #[error("notification database connection failed: {0}")]
    Database(crate::sqlx_notification_database_error::SqlxNotificationDatabaseError),
    #[error("notification metrics recorder initialization failed: {0}")]
    Metrics(crate::metrics_exporter_prometheus_notification_build_error::MetricsExporterPrometheusNotificationBuildError),
    #[error("notification observability initialization failed: {0}")]
    ObservabilityInit(server_observability::observability_init_error::ObservabilityInitError),
    #[error("notification observability shutdown failed: {0}")]
    ObservabilityShutdown(
        server_observability::opentelemetry_sdk_observability_shutdown_error::OpentelemetrySdkObservabilityShutdownError,
    ),
    #[error("notification database migration failed: {0}")]
    Migration(crate::sqlx_notification_migration_error::SqlxNotificationMigrationError),
    #[error("notification service failed: {0}")]
    Serve(server_runtime_http::serve_with_graceful_shutdown_error::ServeWithGracefulShutdownError),
    #[error("notification service socket bind failed: {0}")]
    Socket(crate::notification_io_error::NotificationIoError),
    #[error("notification service timeout configuration is invalid")]
    Timeout,
}
