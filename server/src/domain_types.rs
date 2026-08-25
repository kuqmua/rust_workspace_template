#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct ServerIoError(std::io::Error);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct ServerRuntimeServeError(
    server_runtime_http::domain_types::ServeWithGracefulShutdownError,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct MetricsExporterPrometheusBuildError(metrics_exporter_prometheus::BuildError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct MetricsExporterPrometheusHandle(metrics_exporter_prometheus::PrometheusHandle);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
pub(crate) struct ServerRuntimeRequestTimeoutError(
    server_runtime_http::domain_types::StdRequestTimeoutTryFromDurationError,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
pub(crate) struct ServerRuntimeRunIntervalError(
    server_runtime_http::domain_types::StdRunIntervalTryFromDurationError,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
pub(crate) struct ServerRuntimeBackgroundTaskShutdownError(
    server_runtime_http::domain_types::BackgroundTaskShutdownError,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
pub(crate) struct ServerObservabilityInitError(
    server_runtime_http::domain_types::ObservabilityInitError,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
pub(crate) struct ServerObservabilityShutdownError(
    server_runtime_http::domain_types::OpentelemetrySdkObservabilityShutdownError,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
pub(crate) struct ServerAdminCleanupCfgError(server_admin::domain_types::AdminCleanupCfgError);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminMetricsError {
    #[error(transparent)]
    Render(server_runtime_http::domain_types::MetricsResponseBodyError),
}
impl axum::response::IntoResponse for AdminMetricsError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Render(_error) => axum::response::IntoResponse::into_response(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct ServerConfigError(server_config::domain_types::ConfigTryFromEnvError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct ServerConfigProductionError(server_config::domain_types::ProductionConfigError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct SqlxServerPgConnectError(sqlx::Error);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct ServerAdminMigrateError(server_admin::domain_types::AdminMigrateError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct ServerAdminAuthSvcStateBuildError(
    server_admin::domain_types::auth::AdminAuthSvcStateBuildError,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
pub(crate) struct ServerRuntimeContentSecurityPolicyError(
    server_runtime_http::domain_types::HttpContentSecurityPolicyError,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
pub(crate) struct ServerRuntimeTrustedProxyRangesParseError(
    server_runtime_http::domain_types::TrustedProxyRangesParseError,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub(crate) struct AxumApiRoutes(axum::Router);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::GetInner,
)]
pub(crate) struct HttpBodyMaximumBytes(usize);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::DerefTarget, newtype::FromInner,
)]
pub(crate) struct SharedServerAppStateArc(
    std::sync::Arc<server_app_state::domain_types::ServerAppState<'static>>,
);
impl SharedServerAppStateArc {
    pub(crate) const fn get(
        &self,
    ) -> &std::sync::Arc<server_app_state::domain_types::ServerAppState<'static>> {
        &self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub(crate) struct TokioServerRuntime(tokio::runtime::Runtime);
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(crate) struct ServerExitCode(std::process::ExitCode);
impl std::process::Termination for ServerExitCode {
    fn report(self) -> std::process::ExitCode {
        self.0
    }
}
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
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn administrator_asset_route_preserves_static_file_serving() {
        let response = tower::ServiceExt::oneshot(
            axum::Router::from(server_admin_frontend::domain_types::routes()),
            axum::http::Request::get(constants_str::VALUE_688DB289)
                .body(axum::body::Body::empty())
                .expect("d694b6f6 administrator_asset_route_preserves_static_file_serving invariant must hold"),
        )
        .await
        .expect("499f35e2 administrator_asset_route_preserves_static_file_serving invariant must hold");
        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }

    #[tokio::test]
    async fn operational_routes_are_root_mounted_and_api_routes_are_v1_mounted() {
        let operational_path = common_routes::domain_types::CommonRoute::HealthLive.path();
        let router = axum::Router::from(crate::adapters::routing::mount_service_routes(
            server_runtime_http::domain_types::AxumRouter::from(
                axum::Router::new()
                    .route(
                        operational_path.as_ref(),
                        axum::routing::get(async || axum::http::StatusCode::NO_CONTENT),
                    )
                    .fallback(async || axum::http::StatusCode::IM_A_TEAPOT),
            ),
            super::AxumApiRoutes::from(axum::Router::new().route(
                constants_str::VALUE_87D0B7F8,
                axum::routing::get(async || constants_str::VALUE_14C2529E),
            )),
            super::HttpBodyMaximumBytes::from(1_024usize),
        ))
        .merge(axum::Router::from(
            crate::adapters::routing::frontend_fallback_routes(),
        ));
        let status = |path: &str| {
            tower::ServiceExt::oneshot(
                router.clone(),
                axum::http::Request::builder()
                    .uri(path)
                    .body(axum::body::Body::empty())
                    .expect("7496f84f operational_routes_are_root_mounted_and_api_routes_are_v1_mounted invariant must hold"),
            )
        };
        assert_eq!(
            status(operational_path.as_ref())
                .await
                .expect("0a94fcc5 operational_routes_are_root_mounted_and_api_routes_are_v1_mounted invariant must hold")
                .status(),
            axum::http::StatusCode::NO_CONTENT
        );
        assert_eq!(
            status("/v1/probe").await.expect("6bb8e3f5 operational_routes_are_root_mounted_and_api_routes_are_v1_mounted invariant must hold").status(),
            axum::http::StatusCode::OK
        );
        assert_eq!(
            status("/api/v1/probe").await.expect("11fd3e4a operational_routes_are_root_mounted_and_api_routes_are_v1_mounted invariant must hold").status(),
            axum::http::StatusCode::SEE_OTHER
        );
        assert_eq!(
            status("/v1/health/live").await.expect("6e17db87 operational_routes_are_root_mounted_and_api_routes_are_v1_mounted invariant must hold").status(),
            axum::http::StatusCode::SEE_OTHER
        );
    }

    #[tokio::test]
    async fn missing_page_redirects_to_default_authentication_page() {
        let response = tower::ServiceExt::oneshot(
            axum::Router::from(crate::adapters::routing::frontend_fallback_routes()),
            axum::http::Request::builder()
                .uri(constants_str::VALUE_10D40EF4)
                .body(axum::body::Body::empty())
                .expect("cfe228d8 missing_page_redirects_to_default_authentication_page invariant must hold"),
        )
        .await
        .expect("bd9f2b00 missing_page_redirects_to_default_authentication_page invariant must hold");
        assert_eq!(response.status(), axum::http::StatusCode::SEE_OTHER);
        assert_eq!(
            response.headers().get(axum::http::header::LOCATION),
            Some(&axum::http::HeaderValue::from_static(
                server_admin_contract::domain_types::AdminFrontendPath::SignIn.get()
            ))
        );
    }
    #[test]
    fn tracing_default_filter_is_stable() {
        assert_eq!(constants_str::CONFIG_TRACING_INFO, "info");
    }
}
