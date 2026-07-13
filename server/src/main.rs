const TRACING_DFLT_FILTER: &str = "info";
const ADMIN_CLEANUP_INTERVAL_SECONDS: u64 = 300u64;
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct StdServerIoEr(std::io::Error);
#[derive(Debug)]
struct ServerRuntimeServeEr(server_runtime::ServeWithGracefulShutdownEr);
#[derive(Debug)]
struct MetricsExporterPrometheusBuildEr(metrics_exporter_prometheus::BuildError);
impl std::fmt::Display for MetricsExporterPrometheusBuildEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for MetricsExporterPrometheusBuildEr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Clone, Debug)]
struct MetricsExporterPrometheusHandle(metrics_exporter_prometheus::PrometheusHandle);
#[derive(Debug)]
struct ServerRuntimeRequestTimeoutEr(server_runtime::StdRequestTimeoutTryFromDurationEr);
#[derive(Debug)]
struct ServerRuntimeRunIntervalEr(server_runtime::StdRunIntervalTryFromDurationEr);
#[derive(Debug)]
struct ServerRuntimeBackgroundTaskShutdownEr(server_runtime::BackgroundTaskShutdownEr);
#[derive(Debug)]
struct ServerAdminCleanupCfgEr(server_admin::AdminCleanupCfgEr);
impl std::fmt::Display for ServerRuntimeRequestTimeoutEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for ServerRuntimeRequestTimeoutEr {}
impl std::fmt::Display for ServerRuntimeRunIntervalEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for ServerRuntimeRunIntervalEr {}
impl std::fmt::Display for ServerRuntimeBackgroundTaskShutdownEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for ServerRuntimeBackgroundTaskShutdownEr {}
impl std::fmt::Display for ServerAdminCleanupCfgEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for ServerAdminCleanupCfgEr {}
impl std::fmt::Display for ServerRuntimeServeEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for ServerRuntimeServeEr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct ServerConfigEr(server_config::ConfigTryFromEnvEr);
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct SqlxServerPgConnectEr(sqlx::Error);
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct ServerAdminMigrateEr(server_admin::AdminMigrateEr);
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct ServerPrepPgEr(#[from] server_tbl_example::TblExamplePrepPgEr);
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct ServerAdminAuthSvcStateBuildEr(server_admin::auth::AdminAuthSvcStateBuildEr);
struct AxumApiRoutes(axum::Router);
struct TokioServerRuntime(tokio::runtime::Runtime);
struct StdServerExitCode(std::process::ExitCode);
impl std::process::Termination for StdServerExitCode {
    fn report(self) -> std::process::ExitCode {
        self.0
    }
}
#[derive(Debug, thiserror::Error)]
enum RunServerEr {
    #[error("failed to build administrator authentication state: {0}")]
    AdminAuthState(ServerAdminAuthSvcStateBuildEr),
    #[error("invalid administrator cleanup configuration: {0}")]
    AdminCleanupConfig(ServerAdminCleanupCfgEr),
    #[error("administrator cleanup task shutdown failed: {0}")]
    AdminCleanupShutdown(ServerRuntimeBackgroundTaskShutdownEr),
    #[error("failed to bind service socket: {0}")]
    BindServiceSocket(StdServerIoEr),
    #[error("failed to build tokio runtime: {0}")]
    BuildRuntime(StdServerIoEr),
    #[error("failed to read configuration from environment: {0}")]
    Config(ServerConfigEr),
    #[error("failed to build governor config")]
    GovernorConfig,
    #[error("failed to install metrics recorder: {0}")]
    MetricsRecorder(MetricsExporterPrometheusBuildEr),
    #[error("failed to connect to postgres: {0}")]
    PgConnect(SqlxServerPgConnectEr),
    #[error("failed to prepare administrator schema: {0}")]
    PrepAdminPg(ServerAdminMigrateEr),
    #[error("failed to prepare postgres schema: {0}")]
    PrepPg(ServerPrepPgEr),
    #[error("invalid server runtime interval: {0}")]
    RuntimeInterval(ServerRuntimeRunIntervalEr),
    #[error("invalid server runtime timeout: {0}")]
    RuntimeTimeout(ServerRuntimeRequestTimeoutEr),
    #[error("server failed: {0}")]
    Serve(ServerRuntimeServeEr),
}
#[allow(clippy::single_call_fn)] // keeps validated maintenance policy separate from startup orchestration
fn mk_admin_cleanup_cfg() -> Result<server_admin::AdminCleanupCfg, RunServerEr> {
    let batch_size = server_admin::AdminCleanupBatchSize::try_from(1_000i64)
        .map_err(|error| RunServerEr::AdminCleanupConfig(ServerAdminCleanupCfgEr(error)))?;
    let retention = |seconds| {
        server_admin::AdminCleanupRetentionSeconds::try_from(seconds)
            .map_err(|error| RunServerEr::AdminCleanupConfig(ServerAdminCleanupCfgEr(error)))
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
    };
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
            .merge(axum::Router::from(cmn_routes::cmn_routes(
                cmn_routes::StdArcCmnRoutesAppState::from(std::sync::Arc::<
                    server_app_state::ServerAppState<'static>,
                >::clone(app_state)),
            )))
            .merge(server_tbl_example::TblExample::routes(std::sync::Arc::<
                server_app_state::ServerAppState<'static>,
            >::clone(
                app_state
            )))
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
fn init_tracing() {
    let subscriber = tracing_subscriber::layer::SubscriberExt::with(
        tracing_subscriber::registry(),
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(TRACING_DFLT_FILTER)),
    );
    let subscriber_with_fmt = tracing_subscriber::layer::SubscriberExt::with(
        subscriber,
        tracing_subscriber::fmt::layer(),
    );
    tracing_subscriber::util::SubscriberInitExt::init(subscriber_with_fmt);
}
#[allow(clippy::single_call_fn)] // runtime builder is shared by main and can be reused by startup tests
fn mk_runtime() -> Result<TokioServerRuntime, RunServerEr> {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
        .map(TokioServerRuntime)
        .map_err(|er| RunServerEr::BuildRuntime(StdServerIoEr(er)))
}
#[allow(clippy::single_call_fn)] // isolated pool builder keeps startup flow linear and reuses config getters in one place
async fn mk_pg_pool(config: &server_config::Config) -> Result<app_state::SqlxPgPool, RunServerEr> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(*config_lib::GetPgPoolMaxConnections::get_pg_pool_max_connections(config))
        .connect(secrecy::ExposeSecret::expose_secret(
            config_lib::GetDatabaseUrl::get_database_url(config),
        ))
        .await
        .map(app_state::SqlxPgPool::from)
        .map_err(|er| RunServerEr::PgConnect(SqlxServerPgConnectEr(er)))
}
#[allow(clippy::single_call_fn)] // startup flow is grouped for separation from process/bootstrap concerns
async fn run_server() -> Result<(), RunServerEr> {
    let config = server_config::Config::try_from_env()
        .map_err(|er| RunServerEr::Config(ServerConfigEr(er)))?;
    let pg_pool = mk_pg_pool(&config).await?;
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(pg_pool.as_ref()))
        .await
        .map_err(|er| RunServerEr::PrepAdminPg(ServerAdminMigrateEr(er)))?;
    server_tbl_example::TblExample::prep_pg(pg_pool.as_ref())
        .await
        .map_err(|er| RunServerEr::PrepPg(ServerPrepPgEr::from(er)))?;
    let cleanup_cfg = mk_admin_cleanup_cfg()?;
    let cleanup_interval = server_runtime::StdRunInterval::try_from(
        std::time::Duration::from_secs(ADMIN_CLEANUP_INTERVAL_SECONDS),
    )
    .map_err(|error| RunServerEr::RuntimeInterval(ServerRuntimeRunIntervalEr(error)))?;
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
        return Err(RunServerEr::RuntimeInterval(ServerRuntimeRunIntervalEr(
            server_runtime::StdRunIntervalTryFromDurationEr,
        )));
    };
    let tcp_listener = tokio::net::TcpListener::bind(
        config_lib::GetServiceSocketAddress::get_service_socket_address(&config),
    )
    .await
    .map_err(|er| RunServerEr::BindServiceSocket(StdServerIoEr(er)))?;
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
            .map_err(|er| RunServerEr::AdminAuthState(ServerAdminAuthSvcStateBuildEr(er)))?,
        ));
    let swagger_enabled = *config.admin_swagger_enabled;
    let app_state = mk_app_state(config, pg_pool);
    let metrics_handle = metrics_exporter_prometheus::PrometheusBuilder::new()
        .install_recorder()
        .map(MetricsExporterPrometheusHandle)
        .map_err(|er| RunServerEr::MetricsRecorder(MetricsExporterPrometheusBuildEr(er)))?;
    let api_routes = mk_api_routes(&app_state, admin_auth_state, metrics_handle);
    let governor_conf = std::sync::Arc::new(
        tower_governor::governor::GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(10)
            .finish()
            .ok_or(RunServerEr::GovernorConfig)?,
    );
    let request_timeout =
        server_runtime::StdRequestTimeout::try_from(std::time::Duration::from_secs(30u64))
            .map_err(|er| RunServerEr::RuntimeTimeout(ServerRuntimeRequestTimeoutEr(er)))?;
    let rate_limited_api_routes = api_routes
        .0
        .layer(tower_governor::GovernorLayer::new(governor_conf));
    let router = server_runtime::RequestIdLayer.apply(
        server_runtime::SecurityHeadersLayer::from(server_runtime::ForwardedProtoTrust::Ignore)
            .apply(
                server_runtime::RequestTimeoutLayer::from(request_timeout).apply(
                    server_runtime::AxumRouter::from(
                        axum::Router::new()
                            .nest("/api/v1", rate_limited_api_routes)
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
        async {
            if let Err(er) = tokio::signal::ctrl_c().await {
                eprintln!("failed to wait for ctrl-c signal: {er}");
            }
        },
        request_timeout,
    )
    .await;
    let _cleanup_outcome = cleanup_task
        .shutdown(request_timeout)
        .await
        .map_err(|error| {
            RunServerEr::AdminCleanupShutdown(ServerRuntimeBackgroundTaskShutdownEr(error))
        })?;
    serve_result.map_err(|er| RunServerEr::Serve(ServerRuntimeServeEr(er)))?;
    Ok(())
}
fn main() -> StdServerExitCode {
    init_tracing();
    match mk_runtime().and_then(|runtime| runtime.0.block_on(run_server())) {
        Ok(()) => StdServerExitCode(std::process::ExitCode::SUCCESS),
        Err(er) => {
            eprintln!("{er}");
            StdServerExitCode(std::process::ExitCode::FAILURE)
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn tracing_default_filter_is_stable() {
        assert_eq!(super::TRACING_DFLT_FILTER, "info");
    }
}
