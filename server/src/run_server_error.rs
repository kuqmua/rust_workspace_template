use super::domain_types::{
    MetricsExporterPrometheusBuildError, ServerIoError, SqlxServerPgConnectError,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum RunServerError {
    #[error("failed to build administrator authentication state: {0}")]
    AdminAuthState(server_admin::domain_types::auth::AdminAuthSvcStateBuildError),
    #[error("invalid administrator cleanup configuration: {0}")]
    AdminCleanupConfig(server_admin::domain_types::AdminCleanupCfgError),
    #[error("administrator cleanup task shutdown failed: {0}")]
    AdminCleanupShutdown(server_runtime_http::domain_types::BackgroundTaskShutdownError),
    #[error("failed to bind service socket: {0}")]
    BindServiceSocket(ServerIoError),
    #[error("failed to build tokio runtime: {0}")]
    BuildRuntime(ServerIoError),
    #[error("failed to read configuration from environment: {0}")]
    Config(server_config::config::ConfigTryFromEnvError),
    #[error("unsafe production configuration: {0}")]
    ConfigProduction(server_config::production_config_error::ProductionConfigError),
    #[error("invalid content security policy: {0}")]
    ContentSecurityPolicy(server_runtime_http::domain_types::HttpContentSecurityPolicyError),
    #[error("invalid CORS allow-origin configuration: {0}")]
    CorsAllowOrigin(server_runtime_http::domain_types::HttpCorsAllowOriginHeaderValuesError),
    #[error("failed to install metrics recorder: {0}")]
    MetricsRecorder(MetricsExporterPrometheusBuildError),
    #[error("failed to initialize observability: {0}")]
    ObservabilityInit(server_runtime_http::domain_types::ObservabilityInitError),
    #[error("failed to shut down observability: {0}")]
    ObservabilityShutdown(
        server_runtime_http::domain_types::OpentelemetrySdkObservabilityShutdownError,
    ),
    #[error("failed to connect to postgres: {0}")]
    PgConnect(SqlxServerPgConnectError),
    #[error("postgres minimum connections must not exceed maximum connections")]
    PgPoolConfiguration,
    #[error("failed to prepare administrator schema: {0}")]
    PrepAdminPg(server_admin::domain_types::AdminMigrateError),
    #[error("invalid server runtime interval: {0}")]
    RuntimeInterval(server_runtime_http::domain_types::StdRunIntervalTryFromDurationError),
    #[error("invalid server runtime timeout: {0}")]
    RuntimeTimeout(server_runtime_http::domain_types::StdRequestTimeoutTryFromDurationError),
    #[error("server failed: {0}")]
    Serve(server_runtime_http::domain_types::ServeWithGracefulShutdownError),
    #[error("invalid trusted proxy ranges: {0}")]
    TrustedProxyRanges(server_runtime_http::domain_types::TrustedProxyRangesParseError),
}
