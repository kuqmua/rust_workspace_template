#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum RunServerError {
    #[error("failed to build administrator authentication state: {0}")]
    AdminAuthState(server_admin::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError),
    #[error("invalid administrator cleanup configuration: {0}")]
    AdminCleanupConfig(server_admin::admin_cleanup_cfg_error::AdminCleanupCfgError),
    #[error("administrator cleanup task shutdown failed: {0}")]
    AdminCleanupShutdown(server_runtime_http::background_task_shutdown_error::BackgroundTaskShutdownError),
    #[error("failed to bind service socket: {0}")]
    BindServiceSocket(crate::server_io_error::ServerIoError),
    #[error("failed to build tokio runtime: {0}")]
    BuildRuntime(crate::server_io_error::ServerIoError),
    #[error("failed to read configuration from environment: {0}")]
    Config(server_config::config::ConfigTryFromEnvError),
    #[error("unsafe production configuration: {0}")]
    ConfigProduction(server_config::production_config_error::ProductionConfigError),
    #[error("invalid content security policy: {0}")]
    ContentSecurityPolicy(server_runtime_http::http_content_security_policy_error::HttpContentSecurityPolicyError),
    #[error("invalid CORS allow-origin configuration: {0}")]
    CorsAllowOrigin(server_runtime_http::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError),
    #[error("failed to install metrics recorder: {0}")]
    MetricsRecorder(crate::metrics_exporter_prometheus_build_error::MetricsExporterPrometheusBuildError),
    #[error("failed to initialize observability: {0}")]
    ObservabilityInit(server_observability::observability_init_error::ObservabilityInitError),
    #[error("failed to shut down observability: {0}")]
    ObservabilityShutdown(
        server_observability::opentelemetry_sdk_observability_shutdown_error::OpentelemetrySdkObservabilityShutdownError,
    ),
    #[error("failed to connect to postgres: {0}")]
    PgConnect(crate::sqlx_server_pg_connect_error::SqlxServerPgConnectError),
    #[error("postgres minimum connections must not exceed maximum connections")]
    PgPoolConfiguration,
    #[error("failed to prepare administrator schema: {0}")]
    PrepAdminPg(server_admin::admin_migrate_error::AdminMigrateError),
    #[error("invalid server runtime interval: {0}")]
    RuntimeInterval(server_runtime_http::std_run_interval_try_from_duration_error::StdRunIntervalTryFromDurationError),
    #[error("invalid server runtime timeout: {0}")]
    RuntimeTimeout(server_runtime_http::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError),
    #[error("server failed: {0}")]
    Serve(server_runtime_http::serve_with_graceful_shutdown_error::ServeWithGracefulShutdownError),
    #[error("invalid trusted proxy ranges: {0}")]
    TrustedProxyRanges(server_runtime_http::trusted_proxy_ranges_parse_error::TrustedProxyRangesParseError),
}
