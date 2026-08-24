mod bootstrap;
mod maintenance;
mod routing;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
struct ServerIoError(std::io::Error);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
struct ServerRuntimeServeError(server_runtime_http::ServeWithGracefulShutdownError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
struct MetricsExporterPrometheusBuildError(metrics_exporter_prometheus::BuildError);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
struct MetricsExporterPrometheusHandle(metrics_exporter_prometheus::PrometheusHandle);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
struct ServerRuntimeRequestTimeoutError(server_runtime_http::StdRequestTimeoutTryFromDurationError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
struct ServerRuntimeRunIntervalError(server_runtime_http::StdRunIntervalTryFromDurationError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
struct ServerRuntimeBackgroundTaskShutdownError(server_runtime_http::BackgroundTaskShutdownError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
struct ServerObservabilityInitError(server_runtime_http::ObservabilityInitError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
struct ServerObservabilityShutdownError(
    server_runtime_http::OpentelemetrySdkObservabilityShutdownError,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
struct ServerAdminCleanupCfgError(server_admin::AdminCleanupCfgError);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
enum AdminMetricsError {
    #[error(transparent)]
    Render(server_runtime_http::MetricsResponseBodyError),
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
struct ServerConfigError(server_config::domain_types::ConfigTryFromEnvError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
struct ServerConfigProductionError(server_config::domain_types::ProductionConfigError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
struct SqlxServerPgConnectError(sqlx::Error);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
struct ServerAdminMigrateError(server_admin::AdminMigrateError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
struct ServerAdminAuthSvcStateBuildError(server_admin::auth::AdminAuthSvcStateBuildError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
struct ServerRuntimeContentSecurityPolicyError(server_runtime_http::HttpContentSecurityPolicyError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
struct ServerRuntimeTrustedProxyRangesParseError(server_runtime_http::TrustedProxyRangesParseError);
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct AxumApiRoutes(axum::Router);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct HttpBodyMaximumBytes(usize);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::DerefTarget, newtype::FromInner,
)]
struct SharedServerAppStateArc(
    std::sync::Arc<server_app_state::domain_types::ServerAppState<'static>>,
);
impl SharedServerAppStateArc {
    const fn get(
        &self,
    ) -> &std::sync::Arc<server_app_state::domain_types::ServerAppState<'static>> {
        &self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct TokioServerRuntime(tokio::runtime::Runtime);
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct ServerExitCode(std::process::ExitCode);
impl std::process::Termination for ServerExitCode {
    fn report(self) -> std::process::ExitCode {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
enum RunServerError {
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
    CorsAllowOrigin(server_runtime_http::HttpCorsAllowOriginHeaderValuesError),
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
#[allow(clippy::single_call_fn)] // startup flow is grouped for separation from process/bootstrap concerns
async fn run_server(config: server_config::domain_types::Config) -> Result<(), RunServerError> {
    let pg_pool = bootstrap::mk_pg_pool(&config).await?;
    let cleanup_cfg = maintenance::cfg()?;
    let cleanup_interval = maintenance::interval()?;
    let cleanup_pool = pg_pool.clone();
    let Some(cleanup_task) = server_runtime_http::spawn_interval_task(
        Some(cleanup_interval),
        move || {
            let run_pool = cleanup_pool.clone();
            async move {
                match server_admin::cleanup_admin_tables(
                    app_state::domain_types::SqlxPgPoolRef::from(run_pool.as_ref()),
                    cleanup_cfg,
                )
                .await
                {
                    Ok(report) => tracing::info!(
                        deleted_rows = %report.total_rows(),
                        "administrator operational tables cleaned"
                    ),
                    Err(error) => {
                        tracing::error!(error = %error, "administrator operational table cleanup failed");
                    }
                }
            }
        },
    ) else {
        return Err(RunServerError::RuntimeInterval(
            ServerRuntimeRunIntervalError::from(
                server_runtime_http::StdRunIntervalTryFromDurationError,
            ),
        ));
    };
    let tcp_listener = tokio::net::TcpListener::bind(
        config_lib::GetServiceSocketAddress::get_service_socket_address(&config),
    )
    .await
    .map_err(|error| RunServerError::BindServiceSocket(ServerIoError::from(error)))?;
    let actual_service_socket_address = tcp_listener
        .local_addr()
        .map_err(|error| RunServerError::BindServiceSocket(ServerIoError::from(error)))?;
    tracing::info!(frontend = %actual_service_socket_address);
    let trusted_proxy_ranges = server_runtime_http::parse_trusted_proxy_ranges(
        server_runtime_http::TrustedProxyRangesTextRef::from(
            config.trusted_proxy_ranges_text.0.as_str(),
        ),
    )
    .map_err(|error| {
        RunServerError::TrustedProxyRanges(ServerRuntimeTrustedProxyRangesParseError::from(error))
    })?;
    let cors_origins = Vec::<axum::http::HeaderValue>::from(
        server_runtime_http::parse_cors_allow_origin(
            server_runtime_http::HttpCorsAllowOriginTextRef::from(
                config_lib::GetCorsAllowOrigin::get_cors_allow_origin(&config).as_str(),
            ),
        )
        .map_err(RunServerError::CorsAllowOrigin)?,
    );
    let admin_auth_state =
        server_admin::auth::SharedAdminAuthSvcStateArc::from(std::sync::Arc::new(
            server_admin::auth::AdminAuthSvcState::try_new(
                pg_pool.clone(),
                &config.admin_jwt_secret,
                &config.admin_access_token_ttl_seconds,
                &config.admin_refresh_token_ttl_seconds,
                &config.admin_session_limit,
                &config.admin_sign_in_rate_limit,
                &config.admin_login_failure_limit,
                &config.admin_password_hash_concurrency,
                &config.admin_cookie_secure,
                &config.admin_token_issuer,
                &config.admin_token_audience,
                &config.cors_allow_origin,
            )
            .map_err(|error| {
                RunServerError::AdminAuthState(ServerAdminAuthSvcStateBuildError::from(error))
            })?,
        ));
    let swagger_enabled = *config.admin_swagger_enabled;
    let content_security_policy = server_runtime_http::HttpContentSecurityPolicy::try_from(
        config.content_security_policy.as_ref().to_owned(),
    )
    .map_err(|error| {
        RunServerError::ContentSecurityPolicy(ServerRuntimeContentSecurityPolicyError::from(error))
    })?;
    let maximum_http_body_bytes =
        *config_lib::GetMaximumSizeOfHttpBodyInBytes::get_maximum_size_of_http_body_in_bytes(
            &config,
        );
    let http_gzip_enabled = *config.http_gzip_enabled;
    let request_timeout_seconds = config.request_timeout_seconds.get();
    let app_state = bootstrap::mk_app_state(config, pg_pool);
    let metrics_handle = metrics_exporter_prometheus::PrometheusBuilder::new()
        .install_recorder()
        .map(MetricsExporterPrometheusHandle)
        .map_err(|error| {
            RunServerError::MetricsRecorder(MetricsExporterPrometheusBuildError::from(error))
        })?;
    let admin_html_routes = server_admin::auth::html_routes_with_swagger(
        admin_auth_state.clone(),
        server_admin::auth::AdminHtmlSwaggerEnabled::from(swagger_enabled),
    );
    let html_metrics_handle = metrics_handle.clone();
    let admin_metrics_routes = axum::Router::new()
        .route(
            server_admin_contract::AdminFrontendPath::Metrics.get(),
            axum::routing::get(async move || {
                server_runtime_http::MetricsResponseBody::try_from(html_metrics_handle.0.render())
                    .map_or_else(
                        |_error| {
                            axum::response::IntoResponse::into_response(
                                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            )
                        },
                        |body| {
                            let title_result = server_admin_frontend::ssr::AdminSsrText::try_from(
                                constants_str::METRICS_ALT.to_owned(),
                            );
                            let text_result = server_admin_frontend::ssr::AdminSsrText::try_from(
                                body.into_inner(),
                            );
                            match (title_result, text_result) {
                                (Ok(title), Ok(text)) => {
                                    axum::response::IntoResponse::into_response(
                                        axum::response::Html(String::from(
                                            server_admin_frontend::ssr::render_text_page(
                                                server_admin_contract::AdminPage::Metrics,
                                                title,
                                                text,
                                            ),
                                        )),
                                    )
                                }
                                (Err(_error), _) | (_, Err(_error)) => {
                                    axum::response::IntoResponse::into_response(
                                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                    )
                                }
                            }
                        },
                    )
            }),
        )
        .route_layer(server_admin::AdminGeneratedAuthLayer::from(
            admin_auth_state.clone(),
        ));
    let api_routes = routing::mk_api_routes(&app_state, admin_auth_state, metrics_handle);
    let operational_routes = axum::Router::from(common_routes::common_routes(
        common_routes::ArcCommonRoutesAppState::from(std::sync::Arc::<
            server_app_state::domain_types::ServerAppState<'static>,
        >::clone(app_state.get())),
    ));
    let request_timeout = server_runtime_http::RequestTimeoutDuration::try_from(
        std::time::Duration::from_secs(request_timeout_seconds),
    )
    .map_err(|error| {
        RunServerError::RuntimeTimeout(ServerRuntimeRequestTimeoutError::from(error))
    })?;
    let router = server_runtime_http::RequestIdLayer::with_span_config(
        server_runtime_http::HttpRequestSpanConfig::new(
            server_runtime_http::ServiceName::from(env!("CARGO_PKG_NAME")),
            server_runtime_http::ClientSocketAddr::from(actual_service_socket_address),
            trusted_proxy_ranges,
        ),
    )
    .apply(
        server_runtime_http::HttpMetricsLayer::default().apply(
            server_runtime_http::SecurityHeadersLayer::from(
                server_runtime_http::ForwardedProtoTrust::Ignore,
            )
            .with_content_security_policy(content_security_policy)
            .apply(
                server_runtime_http::RequestTimeoutLayer::from(request_timeout).apply(
                    server_runtime_http::AxumRouter::from(
                        axum::Router::from(routing::mount_service_routes(
                            server_runtime_http::AxumRouter::from(operational_routes),
                            api_routes,
                            HttpBodyMaximumBytes::from(maximum_http_body_bytes),
                        ))
                        .merge(axum::Router::from(server_admin_frontend::routes()))
                        .merge(axum::Router::from(admin_html_routes))
                        .merge(admin_metrics_routes)
                        .merge(axum::Router::from(routing::frontend_fallback_routes()))
                        .layer(
                            tower_http::compression::CompressionLayer::new()
                                .gzip(http_gzip_enabled),
                        )
                        .layer(
                            tower::ServiceBuilder::new().layer(
                                tower_http::cors::CorsLayer::new()
                                    .allow_origin(cors_origins)
                                    .allow_credentials(true)
                                    .allow_headers([
                                        axum::http::header::CONTENT_TYPE,
                                        axum::http::HeaderName::from_static(
                                            constants_str::ROUTE_VALIDATORS_COMMIT_HEADER_NAME,
                                        ),
                                        axum::http::HeaderName::from_static(
                                            constants_str::IDEMPOTENCY_KEY_ALT,
                                        ),
                                        axum::http::HeaderName::from_static(
                                            constants_str::IF_MATCH_ALT,
                                        ),
                                        axum::http::HeaderName::from_static(
                                            constants_str::X_CSRF_TOKEN_ALT,
                                        ),
                                    ])
                                    .allow_methods([
                                        axum::http::Method::GET,
                                        axum::http::Method::POST,
                                        axum::http::Method::PUT,
                                        axum::http::Method::PATCH,
                                        axum::http::Method::DELETE,
                                    ]),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    let serve_result = server_runtime_http::serve_with_graceful_shutdown(
        server_runtime_http::TokioTcpListener::from(tcp_listener),
        router,
        shutdown_signal(),
        request_timeout,
    )
    .await;
    let _cleanup_outcome = cleanup_task
        .shutdown(request_timeout)
        .await
        .map_err(|error| {
            RunServerError::AdminCleanupShutdown(ServerRuntimeBackgroundTaskShutdownError::from(
                error,
            ))
        })?;
    serve_result.map_err(|error| RunServerError::Serve(ServerRuntimeServeError::from(error)))?;
    Ok(())
}
#[allow(
    clippy::single_call_fn,
    reason = "migration mode remains isolated from the long-running service startup path"
)]
async fn migrate_server(
    config: &server_config::domain_types::Config,
) -> Result<(), RunServerError> {
    let pg_pool = bootstrap::mk_pg_pool(config).await?;
    server_admin::prep_pg(app_state::domain_types::SqlxPgPoolRef::from(
        pg_pool.as_ref(),
    ))
    .await
    .map_err(|error| RunServerError::PrepAdminPg(ServerAdminMigrateError::from(error)))
}
#[allow(
    clippy::single_call_fn,
    reason = "the service boundary owns logging for shared signal-installation failures"
)]
async fn shutdown_signal() {
    if let Err(error) = server_runtime_http::wait_for_service_shutdown_signal().await {
        tracing::error!(error = %error, "failed to wait for shutdown signal");
    }
}
fn main() -> ServerExitCode {
    let config = match server_config::domain_types::Config::try_from_env() {
        Ok(config) => config,
        Err(config_error) => {
            let startup_error = RunServerError::Config(ServerConfigError::from(config_error));
            tracing::error!(error = %startup_error, "server configuration failed");
            return ServerExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    if let Err(error) = config.validate_for_startup() {
        tracing::error!(
            error = %RunServerError::ConfigProduction(ServerConfigProductionError::from(error)),
            "server production configuration validation failed"
        );
        return ServerExitCode::from(std::process::ExitCode::FAILURE);
    }
    let tracing_format = if config.tracing_format == config_lib::types::TracingFormat::Json {
        server_runtime_http::ServiceTracingFormat::Json
    } else {
        server_runtime_http::ServiceTracingFormat::Text
    };
    let observability = match server_runtime_http::initialize_service_observability(
        tracing_format,
        server_runtime_http::ServiceName::from(env!("CARGO_PKG_NAME")),
    ) {
        Ok(value) => value,
        Err(error) => {
            tracing::error!(
                error = %RunServerError::ObservabilityInit(ServerObservabilityInitError::from(error)),
                "server observability initialization failed"
            );
            return ServerExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    let run_result = bootstrap::mk_runtime().and_then(|runtime| match config.svc_mode {
        config_lib::types::SvcMode::Migrate => runtime.0.block_on(migrate_server(&config)),
        config_lib::types::SvcMode::Serve => runtime.0.block_on(run_server(config)),
    });
    if let Err(error) = run_result.as_ref() {
        tracing::error!(error = %error, "server terminated with an error");
    }
    let shutdown_result = observability.shutdown().map_err(|error| {
        RunServerError::ObservabilityShutdown(ServerObservabilityShutdownError::from(error))
    });
    match run_result.and(shutdown_result) {
        Ok(()) => ServerExitCode::from(std::process::ExitCode::SUCCESS),
        Err(error) => {
            tracing::error!(error = %error, "server operation or observability shutdown failed");
            ServerExitCode::from(std::process::ExitCode::FAILURE)
        }
    }
}
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn administrator_asset_route_preserves_static_file_serving() {
        let response = tower::ServiceExt::oneshot(
            axum::Router::from(server_admin_frontend::routes()),
            axum::http::Request::get("/admin/assets/style.css")
                .body(axum::body::Body::empty())
                .expect("d694b6f6 administrator_asset_route_preserves_static_file_serving invariant must hold"),
        )
        .await
        .expect("499f35e2 administrator_asset_route_preserves_static_file_serving invariant must hold");
        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }

    #[tokio::test]
    async fn operational_routes_are_root_mounted_and_api_routes_are_v1_mounted() {
        let operational_path = common_routes::CommonRoute::HealthLive.path();
        let router = axum::Router::from(super::routing::mount_service_routes(
            server_runtime_http::AxumRouter::from(
                axum::Router::new()
                    .route(
                        operational_path.as_ref(),
                        axum::routing::get(async || axum::http::StatusCode::NO_CONTENT),
                    )
                    .fallback(async || axum::http::StatusCode::IM_A_TEAPOT),
            ),
            super::AxumApiRoutes::from(
                axum::Router::new().route("/probe", axum::routing::get(async || "api")),
            ),
            super::HttpBodyMaximumBytes::from(1_024usize),
        ))
        .merge(axum::Router::from(
            super::routing::frontend_fallback_routes(),
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
            axum::Router::from(super::routing::frontend_fallback_routes()),
            axum::http::Request::builder()
                .uri("/missing-page")
                .body(axum::body::Body::empty())
                .expect("cfe228d8 missing_page_redirects_to_default_authentication_page invariant must hold"),
        )
        .await
        .expect("bd9f2b00 missing_page_redirects_to_default_authentication_page invariant must hold");
        assert_eq!(response.status(), axum::http::StatusCode::SEE_OTHER);
        assert_eq!(
            response.headers().get(axum::http::header::LOCATION),
            Some(&axum::http::HeaderValue::from_static(
                server_admin_contract::AdminFrontendPath::SignIn.get()
            ))
        );
    }
    #[test]
    fn tracing_default_filter_is_stable() {
        assert_eq!(constants_str::CONFIG_TRACING_INFO, "info");
    }
}
