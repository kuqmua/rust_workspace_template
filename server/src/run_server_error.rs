use crate::domain_types::{
    MetricsExporterPrometheusBuildError, ServerAdminAuthSvcStateBuildError,
    ServerAdminCleanupCfgError, ServerAdminMigrateError, ServerConfigError,
    ServerConfigProductionError, ServerIoError, ServerObservabilityInitError,
    ServerObservabilityShutdownError, ServerRuntimeBackgroundTaskShutdownError,
    ServerRuntimeContentSecurityPolicyError, ServerRuntimeRequestTimeoutError,
    ServerRuntimeRunIntervalError, ServerRuntimeServeError,
    ServerRuntimeTrustedProxyRangesParseError, SqlxServerPgConnectError,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum RunServerError {
    #[error("failed to build administrator authentication state: {0}")]
    AdminAuthState(ServerAdminAuthSvcStateBuildError),
    #[error("invalid administrator cleanup configuration: {0}")]
    AdminCleanupConfig(ServerAdminCleanupCfgError),
    #[error("administrator cleanup task shutdown failed: {0}")]
    AdminCleanupShutdown(ServerRuntimeBackgroundTaskShutdownError),
    #[error("failed to bind service socket: {0}")]
    BindServiceSocket(ServerIoError),
    #[error("failed to build tokio runtime: {0}")]
    BuildRuntime(ServerIoError),
    #[error("failed to read configuration from environment: {0}")]
    Config(ServerConfigError),
    #[error("unsafe production configuration: {0}")]
    ConfigProduction(ServerConfigProductionError),
    #[error("invalid content security policy: {0}")]
    ContentSecurityPolicy(ServerRuntimeContentSecurityPolicyError),
    #[error("invalid CORS allow-origin configuration: {0}")]
    CorsAllowOrigin(server_runtime_http::domain_types::HttpCorsAllowOriginHeaderValuesError),
    #[error("failed to install metrics recorder: {0}")]
    MetricsRecorder(MetricsExporterPrometheusBuildError),
    #[error("failed to initialize observability: {0}")]
    ObservabilityInit(ServerObservabilityInitError),
    #[error("failed to shut down observability: {0}")]
    ObservabilityShutdown(ServerObservabilityShutdownError),
    #[error("failed to connect to postgres: {0}")]
    PgConnect(SqlxServerPgConnectError),
    #[error("postgres minimum connections must not exceed maximum connections")]
    PgPoolConfiguration,
    #[error("failed to prepare administrator schema: {0}")]
    PrepAdminPg(ServerAdminMigrateError),
    #[error("invalid server runtime interval: {0}")]
    RuntimeInterval(ServerRuntimeRunIntervalError),
    #[error("invalid server runtime timeout: {0}")]
    RuntimeTimeout(ServerRuntimeRequestTimeoutError),
    #[error("server failed: {0}")]
    Serve(ServerRuntimeServeError),
    #[error("invalid trusted proxy ranges: {0}")]
    TrustedProxyRanges(ServerRuntimeTrustedProxyRangesParseError),
}
