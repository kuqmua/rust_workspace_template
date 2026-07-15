const ADMIN_CLEANUP_INTERVAL_SECONDS: u64 = 300u64;
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct StdServerIoError(std::io::Error);
#[derive(Debug)]
struct ServerRuntimeServeError(server_runtime::ServeWithGracefulShutdownError);
#[derive(Debug)]
struct MetricsExporterPrometheusBuildError(metrics_exporter_prometheus::BuildError);
impl std::fmt::Display for MetricsExporterPrometheusBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for MetricsExporterPrometheusBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Clone, Debug)]
struct MetricsExporterPrometheusHandle(metrics_exporter_prometheus::PrometheusHandle);
#[derive(Debug)]
struct ServerRuntimeRequestTimeoutError(server_runtime::StdRequestTimeoutTryFromDurationError);
#[derive(Debug)]
struct ServerRuntimeRunIntervalError(server_runtime::StdRunIntervalTryFromDurationError);
#[derive(Debug)]
struct ServerRuntimeBackgroundTaskShutdownError(server_runtime::BackgroundTaskShutdownError);
#[derive(Debug)]
struct ServerAdminCleanupCfgError(server_admin::AdminCleanupCfgError);
impl std::fmt::Display for ServerRuntimeRequestTimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for ServerRuntimeRequestTimeoutError {}
impl std::fmt::Display for ServerRuntimeRunIntervalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for ServerRuntimeRunIntervalError {}
impl std::fmt::Display for ServerRuntimeBackgroundTaskShutdownError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for ServerRuntimeBackgroundTaskShutdownError {}
impl std::fmt::Display for ServerAdminCleanupCfgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for ServerAdminCleanupCfgError {}
impl std::fmt::Display for ServerRuntimeServeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for ServerRuntimeServeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct ServerConfigError(server_config::ConfigTryFromEnvError);
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct SqlxServerPgConnectError(sqlx::Error);
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct ServerAdminMigrateError(server_admin::AdminMigrateError);
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct ServerPrepPgError(#[from] server_table_example::TableExamplePrepPgError);
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct ServerAdminAuthSvcStateBuildError(server_admin::auth::AdminAuthSvcStateBuildError);
struct AxumApiRoutes(axum::Router);
#[derive(Clone, Debug)]
struct ClientIpRateLimitKeyExtractor {
    trusted_proxy_ranges: server_runtime::TrustedProxyRanges,
}
impl tower_governor::key_extractor::KeyExtractor for ClientIpRateLimitKeyExtractor {
    type Key = std::net::IpAddr;
    fn extract<Body>(
        &self,
        req: &axum::http::Request<Body>,
    ) -> Result<Self::Key, tower_governor::errors::GovernorError> {
        let peer = req
            .extensions()
            .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
            .map(|value| value.0)
            .ok_or(tower_governor::errors::GovernorError::UnableToExtractKey)?;
        Ok(*server_runtime::resolve_client_ip(
            server_runtime::HttpHeaderMapRef::from(req.headers()),
            server_runtime::StdSocketAddr::from(peer),
            &self.trusted_proxy_ranges,
        )
        .as_ref())
    }
}
struct TokioServerRuntime(tokio::runtime::Runtime);
struct StdServerExitCode(std::process::ExitCode);
impl std::process::Termination for StdServerExitCode {
    fn report(self) -> std::process::ExitCode {
        self.0
    }
}
#[derive(Debug, thiserror::Error)]
enum RunServerError {
    #[error("failed to build administrator authentication state: {0}")]
    AdminAuthState(ServerAdminAuthSvcStateBuildError),
    #[error("invalid administrator cleanup configuration: {0}")]
    AdminCleanupConfig(ServerAdminCleanupCfgError),
    #[error("administrator cleanup task shutdown failed: {0}")]
    AdminCleanupShutdown(ServerRuntimeBackgroundTaskShutdownError),
    #[error("failed to bind service socket: {0}")]
    BindServiceSocket(StdServerIoError),
    #[error("failed to build tokio runtime: {0}")]
    BuildRuntime(StdServerIoError),
    #[error("failed to read configuration from environment: {0}")]
    Config(ServerConfigError),
    #[error("failed to build governor config")]
    GovernorConfig,
    #[error("failed to install metrics recorder: {0}")]
    MetricsRecorder(MetricsExporterPrometheusBuildError),
    #[error("failed to connect to postgres: {0}")]
    PgConnect(SqlxServerPgConnectError),
    #[error("failed to prepare administrator schema: {0}")]
    PrepAdminPg(ServerAdminMigrateError),
    #[error("failed to prepare postgres schema: {0}")]
    PrepPg(ServerPrepPgError),
    #[error("invalid server runtime interval: {0}")]
    RuntimeInterval(ServerRuntimeRunIntervalError),
    #[error("invalid server runtime timeout: {0}")]
    RuntimeTimeout(ServerRuntimeRequestTimeoutError),
    #[error("server failed: {0}")]
    Serve(ServerRuntimeServeError),
    #[error("invalid trusted proxy range: {0}")]
    TrustedProxyRange(server_runtime::TrustedProxyRangeParseError),
}
#[allow(clippy::single_call_fn)] // keeps validated maintenance policy separate from startup orchestration
fn mk_admin_cleanup_cfg() -> Result<server_admin::AdminCleanupCfg, RunServerError> {
    let batch_size = server_admin::AdminCleanupBatchSize::try_from(1_000i64)
        .map_err(|error| RunServerError::AdminCleanupConfig(ServerAdminCleanupCfgError(error)))?;
    let retention = |seconds| {
        server_admin::AdminCleanupRetentionSeconds::try_from(seconds)
            .map_err(|error| RunServerError::AdminCleanupConfig(ServerAdminCleanupCfgError(error)))
    };
    Ok(server_admin::AdminCleanupCfg::new(
        batch_size,
        retention(604_800i64)?,
        retention(7_776_000i64)?,
        retention(86_400i64)?,
        retention(86_400i64)?,
        retention(3_600i64)?,
    ))
}
#[allow(clippy::single_call_fn)] // route wiring is reused by startup flow and isolated from layer setup
fn mk_api_routes(
    app_state: &std::sync::Arc<server_app_state::ServerAppState<'static>>,
    admin_auth_state: server_admin::auth::StdSharedAdminAuthSvcState,
    metrics_handle: MetricsExporterPrometheusHandle,
) -> AxumApiRoutes {
    let generated_admin_auth_state = admin_auth_state.clone();
    let generated_table_routes =
        server_admin::generated_tables::AdminRoles::routes(std::sync::Arc::<
            server_app_state::ServerAppState<'static>,
        >::clone(app_state))
        .merge(
            server_admin::generated_tables::AdminRolePermissions::routes(std::sync::Arc::<
                server_app_state::ServerAppState<'static>,
            >::clone(
                app_state
            )),
        )
        .merge(server_admin::generated_tables::AdminPermissions::routes(
            std::sync::Arc::<server_app_state::ServerAppState<'static>>::clone(app_state),
        ))
        .merge(server_admin::generated_tables::AdminSystemSettings::routes(
            std::sync::Arc::<server_app_state::ServerAppState<'static>>::clone(app_state),
        ))
        .merge(server_admin::generated_tables::AdminUsers::routes(
            std::sync::Arc::<server_app_state::ServerAppState<'static>>::clone(app_state),
        ))
        .merge(server_admin::generated_tables::AdminUserRoles::routes(
            std::sync::Arc::<server_app_state::ServerAppState<'static>>::clone(app_state),
        ));
    let documented_admin_routes = if *app_state.config.admin_swagger_enabled {
        generated_table_routes.route(
            "/openapi.json",
            axum::routing::get(async || {
                axum::Json(utoipa::openapi::OpenApi::from(
                    server_admin::generated_tables::generated_open_api(),
                ))
            }),
        )
    } else {
        generated_table_routes
    }
    .method_not_allowed_fallback(async || server_admin::auth::AdminApiError::MethodNotAllowed);
    let secured_admin_routes = documented_admin_routes
        .route(
            "/metrics",
            axum::routing::get(async move || metrics_handle.0.render()),
        )
        .route_layer(server_admin::AdminGeneratedAuthLayer::from(
            generated_admin_auth_state,
        ));
    AxumApiRoutes(
        axum::Router::new()
            .merge(server_table_example::TableExample::routes(
                std::sync::Arc::<server_app_state::ServerAppState<'static>>::clone(app_state),
            ))
            .nest(
                "/admin",
                axum::Router::from(server_admin::auth::routes(admin_auth_state)),
            )
            .nest("/admin", secured_admin_routes),
    )
}
#[allow(clippy::single_call_fn)] // keeps state creation shape reusable and type-stable in one place
fn mk_app_state(
    config: server_config::Config,
    pg_pool: app_state::SqlxPgPool,
) -> std::sync::Arc<server_app_state::ServerAppState<'static>> {
    std::sync::Arc::new(server_app_state::ServerAppState {
        bulk_item_budget: server_runtime::ResourceBudget::new(
            server_runtime::ResourceBudgetMaximum::from(
                std::num::NonZeroUsize::new(4_096usize).unwrap_or(std::num::NonZeroUsize::MIN),
            ),
        ),
        config,
        idempotency_response_budget: server_runtime::ResourceBudget::new(
            server_runtime::ResourceBudgetMaximum::from(
                std::num::NonZeroUsize::new(64usize.saturating_mul(1_048_576usize))
                    .unwrap_or(std::num::NonZeroUsize::MIN),
            ),
        ),
        pg_pool,
        project_git_info: &git_info::PROJECT_GIT_INFO,
    })
}
#[allow(clippy::single_call_fn)] // tracing initialization is split out so runtime bootstrap stays focused
fn initialization_tracing() {
    let subscriber = tracing_subscriber::layer::SubscriberExt::with(
        tracing_subscriber::registry(),
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            tracing_subscriber::EnvFilter::new(contract_constants::server::TRACING_DFLT_FILTER)
        }),
    );
    let subscriber_with_fmt = tracing_subscriber::layer::SubscriberExt::with(
        subscriber,
        tracing_subscriber::fmt::layer(),
    );
    tracing_subscriber::util::SubscriberInitExt::init(subscriber_with_fmt);
}
#[allow(clippy::single_call_fn)] // runtime builder is shared by main and can be reused by startup tests
fn mk_runtime() -> Result<TokioServerRuntime, RunServerError> {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
        .map(TokioServerRuntime)
        .map_err(|error| RunServerError::BuildRuntime(StdServerIoError(error)))
}
#[allow(clippy::single_call_fn)] // isolated pool builder keeps startup flow linear and reuses config getters in one place
async fn mk_pg_pool(
    config: &server_config::Config,
) -> Result<app_state::SqlxPgPool, RunServerError> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(*config_lib::GetPgPoolMaxConnections::get_pg_pool_max_connections(config))
        .connect(secrecy::ExposeSecret::expose_secret(
            config_lib::GetDatabaseUrl::get_database_url(config),
        ))
        .await
        .map(app_state::SqlxPgPool::from)
        .map_err(|error| RunServerError::PgConnect(SqlxServerPgConnectError(error)))
}
#[allow(clippy::single_call_fn)] // startup flow is grouped for separation from process/bootstrap concerns
async fn run_server() -> Result<(), RunServerError> {
    let config = server_config::Config::try_from_env()
        .map_err(|error| RunServerError::Config(ServerConfigError(error)))?;
    let pg_pool = mk_pg_pool(&config).await?;
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(pg_pool.as_ref()))
        .await
        .map_err(|error| RunServerError::PrepAdminPg(ServerAdminMigrateError(error)))?;
    server_table_example::TableExample::prep_pg(pg_pool.as_ref())
        .await
        .map_err(|error| RunServerError::PrepPg(ServerPrepPgError::from(error)))?;
    let cleanup_cfg = mk_admin_cleanup_cfg()?;
    let cleanup_interval = server_runtime::StdRunInterval::try_from(
        std::time::Duration::from_secs(ADMIN_CLEANUP_INTERVAL_SECONDS),
    )
    .map_err(|error| RunServerError::RuntimeInterval(ServerRuntimeRunIntervalError(error)))?;
    let cleanup_pool = pg_pool.clone();
    let Some(cleanup_task) = server_runtime::spawn_interval_task(
        Some(cleanup_interval),
        move || {
            let run_pool = cleanup_pool.clone();
            async move {
                match server_admin::cleanup_admin_tables(
                    app_state::SqlxPgPoolRef::from(run_pool.as_ref()),
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
            ServerRuntimeRunIntervalError(server_runtime::StdRunIntervalTryFromDurationError),
        ));
    };
    let tcp_listener = tokio::net::TcpListener::bind(
        config_lib::GetServiceSocketAddress::get_service_socket_address(&config),
    )
    .await
    .map_err(|error| RunServerError::BindServiceSocket(StdServerIoError(error)))?;
    let cors_origins = Vec::<axum::http::HeaderValue>::from(
        server_runtime::parse_cors_allow_origin(server_runtime::HttpCorsAllowOriginTextRef::from(
            config_lib::GetCorsAllowOrigin::get_cors_allow_origin(&config).as_str(),
        )),
    );
    let admin_auth_state =
        server_admin::auth::StdSharedAdminAuthSvcState::from(std::sync::Arc::new(
            server_admin::auth::AdminAuthSvcState::try_new(
                pg_pool.clone(),
                &config.admin_jwt_secret,
                &config.admin_access_token_ttl_seconds,
                &config.admin_refresh_token_ttl_seconds,
                &config.admin_session_limit,
                &config.admin_sign_in_rate_limit,
                &config.admin_password_hash_concurrency,
                &config.admin_cookie_secure,
                &config.admin_token_issuer,
                &config.admin_token_audience,
                &config.cors_allow_origin,
            )
            .map_err(|error| {
                RunServerError::AdminAuthState(ServerAdminAuthSvcStateBuildError(error))
            })?,
        ));
    let swagger_enabled = *config.admin_swagger_enabled;
    let trusted_proxy_ranges = config
        .trusted_proxy_ranges_text
        .0
        .split(',')
        .map(str::trim)
        .map(str::to_owned)
        .map(server_runtime::TrustedProxyRange::try_from)
        .collect::<Result<Vec<server_runtime::TrustedProxyRange>, _>>()
        .map(server_runtime::TrustedProxyRanges::from)
        .map_err(RunServerError::TrustedProxyRange)?;
    let app_state = mk_app_state(config, pg_pool);
    let metrics_handle = metrics_exporter_prometheus::PrometheusBuilder::new()
        .install_recorder()
        .map(MetricsExporterPrometheusHandle)
        .map_err(|error| {
            RunServerError::MetricsRecorder(MetricsExporterPrometheusBuildError(error))
        })?;
    let api_routes = mk_api_routes(&app_state, admin_auth_state, metrics_handle);
    let operational_routes = axum::Router::from(common_routes::common_routes(
        common_routes::StdArcCommonRoutesAppState::from(std::sync::Arc::<
            server_app_state::ServerAppState<'static>,
        >::clone(&app_state)),
    ));
    let governor_conf = std::sync::Arc::new(
        tower_governor::governor::GovernorConfigBuilder::default()
            .key_extractor(ClientIpRateLimitKeyExtractor {
                trusted_proxy_ranges,
            })
            .per_second(2)
            .burst_size(10)
            .finish()
            .ok_or(RunServerError::GovernorConfig)?,
    );
    let request_timeout =
        server_runtime::StdRequestTimeout::try_from(std::time::Duration::from_secs(30u64))
            .map_err(|error| {
                RunServerError::RuntimeTimeout(ServerRuntimeRequestTimeoutError(error))
            })?;
    let rate_limited_api_routes = api_routes
        .0
        .layer(tower_governor::GovernorLayer::new(governor_conf));
    let router = server_runtime::RequestIdLayer.apply(
        server_runtime::SecurityHeadersLayer::from(server_runtime::ForwardedProtoTrust::Ignore)
            .apply(
                server_runtime::RequestTimeoutLayer::from(request_timeout).apply(
                    server_runtime::AxumRouter::from(
                        axum::Router::new()
                            .nest("/api/v1", operational_routes.merge(rate_limited_api_routes))
                            .merge(axum::Router::from(if swagger_enabled {
                                server_admin_frontend::routes()
                            } else {
                                server_admin_frontend::routes_without_swagger()
                            }))
                            .layer(
                                tower::ServiceBuilder::new().layer(
                                    tower_http::cors::CorsLayer::new()
                                        .allow_origin(cors_origins)
                                        .allow_credentials(true)
                                        .allow_headers([
                                            axum::http::header::CONTENT_TYPE,
                                            axum::http::HeaderName::from_static("commit"),
                                            axum::http::HeaderName::from_static("idempotency-key"),
                                            axum::http::HeaderName::from_static("if-match"),
                                            axum::http::HeaderName::from_static("x-csrf-token"),
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
    );
    let serve_result = server_runtime::serve_with_graceful_shutdown(
        server_runtime::TokioTcpListener::from(tcp_listener),
        router,
        shutdown_signal(),
        request_timeout,
    )
    .await;
    let _cleanup_outcome = cleanup_task
        .shutdown(request_timeout)
        .await
        .map_err(|error| {
            RunServerError::AdminCleanupShutdown(ServerRuntimeBackgroundTaskShutdownError(error))
        })?;
    serve_result.map_err(|error| RunServerError::Serve(ServerRuntimeServeError(error)))?;
    Ok(())
}
#[cfg(not(unix))]
#[allow(clippy::single_call_fn)] // shutdown signal ownership stays isolated from server assembly
async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::error!(error = %error, "failed to wait for shutdown signal");
    }
}
#[cfg(unix)]
#[allow(
    clippy::integer_division_remainder_used,
    clippy::single_call_fn,
    reason = "tokio::select macro internals trigger the remainder lint; shutdown signal ownership stays isolated"
)]
async fn shutdown_signal() {
    let terminate = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate());
    match terminate {
        Ok(mut signal) => {
            tokio::select! {
                ctrl_c = tokio::signal::ctrl_c() => {
                    if let Err(error) = ctrl_c {
                        tracing::error!(error = %error, "failed to wait for ctrl-c signal");
                    }
                }
                _signal = signal.recv() => {}
            }
        }
        Err(error) => {
            tracing::error!(error = %error, "failed to install SIGTERM handler");
            if let Err(ctrl_c_error) = tokio::signal::ctrl_c().await {
                tracing::error!(error = %ctrl_c_error, "failed to wait for ctrl-c signal");
            }
        }
    }
}
fn main() -> StdServerExitCode {
    initialization_tracing();
    match mk_runtime().and_then(|runtime| runtime.0.block_on(run_server())) {
        Ok(()) => StdServerExitCode(std::process::ExitCode::SUCCESS),
        Err(error) => {
            eprintln!("{error}");
            StdServerExitCode(std::process::ExitCode::FAILURE)
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn rate_limit_key_uses_forwarded_client_only_for_trusted_proxy() {
        let extractor = super::ClientIpRateLimitKeyExtractor {
            trusted_proxy_ranges: server_runtime::TrustedProxyRanges::from(vec![
                server_runtime::TrustedProxyRange::try_from("127.0.0.1/32".to_owned())
                    .expect("5c81d907"),
            ]),
        };
        let mut request = axum::http::Request::builder()
            .header("x-forwarded-for", "203.0.113.9")
            .body(())
            .expect("b2604d91");
        let _previous_peer = request.extensions_mut().insert(axum::extract::ConnectInfo(
            std::net::SocketAddr::from((std::net::Ipv4Addr::LOCALHOST, 4_321u16)),
        ));
        assert_eq!(
            tower_governor::key_extractor::KeyExtractor::extract(&extractor, &request)
                .expect("a97e5b21"),
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(203u8, 0u8, 113u8, 9u8))
        );
    }
    #[test]
    fn tracing_default_filter_is_stable() {
        assert_eq!(contract_constants::server::TRACING_DFLT_FILTER, "info");
    }
}
