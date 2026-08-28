use super::{
    MetricsExporterPrometheusNotificationBuildError, NotificationConfigError, NotificationIoError,
    NotificationObservabilityInitError, NotificationObservabilityShutdownError,
    NotificationServeError, SqlxNotificationDatabaseError, SqlxNotificationMigrationError,
};

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
