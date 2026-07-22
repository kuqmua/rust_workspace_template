const ADMIN_CLEANUP_INTERVAL_SECONDS: u64 = 300u64;
#[derive(Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
struct StdServerIoError(std::io::Error);
#[derive(Debug, newtype::FromInner, newtype::Display)]
struct ServerRuntimeServeError(server_runtime::ServeWithGracefulShutdownError);
#[derive(Debug, newtype::FromInner, newtype::Display)]
struct MetricsExporterPrometheusBuildError(metrics_exporter_prometheus::BuildError);

impl std::error::Error for MetricsExporterPrometheusBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Clone, Debug, newtype::FromInner)]
struct MetricsExporterPrometheusHandle(metrics_exporter_prometheus::PrometheusHandle);
#[derive(Debug, newtype::FromInner, newtype::Display)]
struct ServerRuntimeRequestTimeoutError(server_runtime::StdRequestTimeoutTryFromDurationError);
#[derive(Debug, newtype::FromInner, newtype::Display)]
struct ServerRuntimeRunIntervalError(server_runtime::StdRunIntervalTryFromDurationError);
#[derive(Debug, newtype::FromInner, newtype::Display)]
struct ServerRuntimeBackgroundTaskShutdownError(server_runtime::BackgroundTaskShutdownError);
#[derive(Debug, newtype::FromInner, newtype::Display)]
struct ServerAdminCleanupCfgError(server_admin::AdminCleanupCfgError);

impl std::error::Error for ServerRuntimeRequestTimeoutError {}

impl std::error::Error for ServerRuntimeRunIntervalError {}

impl std::error::Error for ServerRuntimeBackgroundTaskShutdownError {}

impl std::error::Error for ServerAdminCleanupCfgError {}

impl std::error::Error for ServerRuntimeServeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
struct ServerConfigError(server_config::ConfigTryFromEnvError);
#[derive(Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
struct SqlxServerPgConnectError(sqlx::Error);
#[derive(Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
struct ServerAdminMigrateError(server_admin::AdminMigrateError);
#[derive(Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
struct ServerAdminAuthSvcStateBuildError(server_admin::auth::AdminAuthSvcStateBuildError);
#[derive(Debug, newtype::FromInner, newtype::Display)]
struct ServerRuntimeContentSecurityPolicyError(server_runtime::HttpContentSecurityPolicyError);

impl std::error::Error for ServerRuntimeContentSecurityPolicyError {}
#[derive(newtype::FromInner)]
struct AxumApiRoutes(axum::Router);
#[derive(Clone, newtype::FromInner)]
struct StdSharedServerAppState(std::sync::Arc<server_app_state::ServerAppState<'static>>);
impl StdSharedServerAppState {
    const fn get(&self) -> &std::sync::Arc<server_app_state::ServerAppState<'static>> {
        &self.0
    }
}
impl std::ops::Deref for StdSharedServerAppState {
    type Target = server_app_state::ServerAppState<'static>;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
#[derive(newtype::FromInner)]
struct TokioServerRuntime(tokio::runtime::Runtime);
#[derive(newtype::FromInner)]
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
    #[error("invalid content security policy: {0}")]
    ContentSecurityPolicy(ServerRuntimeContentSecurityPolicyError),
    #[error("invalid CORS allow-origin configuration: {0}")]
    CorsAllowOrigin(server_runtime::HttpCorsAllowOriginHeaderValuesError),
    #[error("failed to install metrics recorder: {0}")]
    MetricsRecorder(MetricsExporterPrometheusBuildError),
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
}
#[allow(clippy::single_call_fn)] // keeps validated maintenance policy separate from startup orchestration
fn mk_admin_cleanup_cfg() -> Result<server_admin::AdminCleanupCfg, RunServerError> {
    let batch_size = server_admin::AdminCleanupBatchSize::try_from(1_000i64).map_err(|error| {
        RunServerError::AdminCleanupConfig(ServerAdminCleanupCfgError::from(error))
    })?;
    let retention = |seconds| {
        server_admin::AdminCleanupRetentionSeconds::try_from(seconds).map_err(|error| {
            RunServerError::AdminCleanupConfig(ServerAdminCleanupCfgError::from(error))
        })
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
#[allow(clippy::single_call_fn)] // isolates the fallback router for an end-to-end routing test
fn frontend_fallback_routes() -> server_runtime::AxumRouter {
    server_runtime::AxumRouter::from(axum::Router::new().fallback(async || {
        axum::response::Redirect::to(server_admin_contract::AdminFrontendPath::SignIn.get())
    }))
}
#[allow(clippy::single_call_fn)] // route wiring is reused by startup flow and isolated from layer setup
fn mk_api_routes(
    app_state: &StdSharedServerAppState,
    admin_auth_state: server_admin::auth::StdSharedAdminAuthSvcState,
    metrics_handle: MetricsExporterPrometheusHandle,
) -> AxumApiRoutes {
    let generated_admin_auth_state = admin_auth_state.clone();
    let generated_table_routes =
        server_admin::generated_tables::AdminRoles::routes(std::sync::Arc::<
            server_app_state::ServerAppState<'static>,
        >::clone(app_state.get()))
        .merge(
            server_admin::generated_tables::AdminRolePermissions::routes(std::sync::Arc::<
                server_app_state::ServerAppState<'static>,
            >::clone(
                app_state.get()
            )),
        )
        .merge(server_admin::generated_tables::AdminPermissions::routes(
            std::sync::Arc::<server_app_state::ServerAppState<'static>>::clone(app_state.get()),
        ))
        .merge(server_admin::generated_tables::AdminSystemSettings::routes(
            std::sync::Arc::<server_app_state::ServerAppState<'static>>::clone(app_state.get()),
        ))
        .merge(server_admin::generated_tables::AdminUsers::routes(
            std::sync::Arc::<server_app_state::ServerAppState<'static>>::clone(app_state.get()),
        ))
        .merge(server_admin::generated_tables::AdminUserRoles::routes(
            std::sync::Arc::<server_app_state::ServerAppState<'static>>::clone(app_state.get()),
        ));
    let documented_admin_routes = if *app_state.config.admin_swagger_enabled {
        generated_table_routes.route(
            str_constants::OPENAPI_JSON,
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
            str_constants::METRICS,
            axum::routing::get(async move || {
                match server_runtime::MetricsResponseBody::try_from(metrics_handle.0.render()) {
                    Ok(body) => axum::response::IntoResponse::into_response((
                        axum::http::StatusCode::OK,
                        body.into_inner(),
                    )),
                    Err(_error) => axum::response::IntoResponse::into_response(
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    ),
                }
            }),
        )
        .route_layer(server_admin::AdminGeneratedAuthLayer::from(
            generated_admin_auth_state,
        ));
    AxumApiRoutes::from(
        axum::Router::new()
            .nest(
                server_admin_contract::AdminFrontendPath::Root.get(),
                axum::Router::from(server_admin::auth::routes(admin_auth_state)),
            )
            .nest(
                server_admin_contract::AdminFrontendPath::Root.get(),
                secured_admin_routes,
            ),
    )
}
#[allow(clippy::single_call_fn)] // keeps state creation shape reusable and type-stable in one place
fn mk_app_state(
    config: server_config::Config,
    pg_pool: app_state::SqlxPgPool,
) -> StdSharedServerAppState {
    StdSharedServerAppState::from(std::sync::Arc::new(server_app_state::ServerAppState {
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
        project_git_info: git_info::project_git_info(),
    }))
}
#[allow(clippy::single_call_fn)] // tracing initialization is split out so runtime bootstrap stays focused
fn initialization_tracing(format: config_lib::types::TracingFormat) {
    let subscriber = tracing_subscriber::layer::SubscriberExt::with(
        tracing_subscriber::registry(),
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            tracing_subscriber::EnvFilter::new(str_constants::CONFIG_TRACING_INFO)
        }),
    );
    if format == config_lib::types::TracingFormat::Json {
        tracing_subscriber::util::SubscriberInitExt::init(
            tracing_subscriber::layer::SubscriberExt::with(
                subscriber,
                tracing_subscriber::fmt::layer().json(),
            ),
        );
    } else {
        tracing_subscriber::util::SubscriberInitExt::init(
            tracing_subscriber::layer::SubscriberExt::with(
                subscriber,
                tracing_subscriber::fmt::layer(),
            ),
        );
    }
}
#[allow(clippy::single_call_fn)] // runtime builder is shared by main and can be reused by startup tests
fn mk_runtime() -> Result<TokioServerRuntime, RunServerError> {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
        .map(TokioServerRuntime)
        .map_err(|error| RunServerError::BuildRuntime(StdServerIoError::from(error)))
}
#[allow(clippy::single_call_fn)] // isolated pool builder keeps startup flow linear and reuses config getters in one place
async fn mk_pg_pool(
    config: &server_config::Config,
) -> Result<app_state::SqlxPgPool, RunServerError> {
    if *config.pg_pool_min_connections
        > *config_lib::GetPgPoolMaxConnections::get_pg_pool_max_connections(config)
    {
        return Err(RunServerError::PgPoolConfiguration);
    }
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(*config_lib::GetPgPoolMaxConnections::get_pg_pool_max_connections(config))
        .min_connections(*config.pg_pool_min_connections)
        .acquire_timeout(std::time::Duration::from_secs(
            config.pg_pool_acquire_timeout_seconds.get(),
        ))
        .idle_timeout(std::time::Duration::from_secs(
            config.pg_pool_idle_timeout_seconds.get(),
        ))
        .max_lifetime(std::time::Duration::from_secs(
            config.pg_pool_max_lifetime_seconds.get(),
        ))
        .after_connect(|connection, _metadata| {
            Box::pin(async move {
                sqlx::Executor::execute(
                    &mut *connection,
                    str_constants::POSTGRES_STATEMENT_TIMEOUT_SQL,
                )
                .await
                .map(drop)
            })
        })
        .connect(secrecy::ExposeSecret::expose_secret(
            config_lib::GetDatabaseUrl::get_database_url(config),
        ))
        .await
        .map(app_state::SqlxPgPool::from)
        .map_err(|error| RunServerError::PgConnect(SqlxServerPgConnectError::from(error)))
}
#[allow(clippy::single_call_fn)] // startup flow is grouped for separation from process/bootstrap concerns
async fn run_server(config: server_config::Config) -> Result<(), RunServerError> {
    let pg_pool = mk_pg_pool(&config).await?;
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(pg_pool.as_ref()))
        .await
        .map_err(|error| RunServerError::PrepAdminPg(ServerAdminMigrateError::from(error)))?;
    let cleanup_cfg = mk_admin_cleanup_cfg()?;
    let cleanup_interval = server_runtime::StdRunInterval::try_from(
        std::time::Duration::from_secs(ADMIN_CLEANUP_INTERVAL_SECONDS),
    )
    .map_err(|error| RunServerError::RuntimeInterval(ServerRuntimeRunIntervalError::from(error)))?;
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
            ServerRuntimeRunIntervalError::from(server_runtime::StdRunIntervalTryFromDurationError),
        ));
    };
    let service_socket_address =
        config_lib::GetServiceSocketAddress::get_service_socket_address(&config);
    let tcp_listener = tokio::net::TcpListener::bind(service_socket_address)
        .await
        .map_err(|error| RunServerError::BindServiceSocket(StdServerIoError::from(error)))?;
    tracing::info!(frontend = %service_socket_address);
    let cors_origins = Vec::<axum::http::HeaderValue>::from(
        server_runtime::parse_cors_allow_origin(server_runtime::HttpCorsAllowOriginTextRef::from(
            config_lib::GetCorsAllowOrigin::get_cors_allow_origin(&config).as_str(),
        ))
        .map_err(RunServerError::CorsAllowOrigin)?,
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
                RunServerError::AdminAuthState(ServerAdminAuthSvcStateBuildError::from(error))
            })?,
        ));
    let swagger_enabled = *config.admin_swagger_enabled;
    let content_security_policy = server_runtime::HttpContentSecurityPolicy::try_from(
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
    let app_state = mk_app_state(config, pg_pool);
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
                server_runtime::MetricsResponseBody::try_from(html_metrics_handle.0.render())
                    .map_or_else(
                        |_error| {
                            axum::response::IntoResponse::into_response(
                                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            )
                        },
                        |body| {
                            let title_result = server_admin_frontend::ssr::AdminSsrText::try_from(
                                str_constants::METRICS_ALT.to_owned(),
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
    let api_routes = mk_api_routes(&app_state, admin_auth_state, metrics_handle);
    let operational_routes = axum::Router::from(common_routes::common_routes(
        common_routes::StdArcCommonRoutesAppState::from(std::sync::Arc::<
            server_app_state::ServerAppState<'static>,
        >::clone(app_state.get())),
    ));
    let request_timeout =
        server_runtime::StdRequestTimeout::try_from(std::time::Duration::from_secs(30u64))
            .map_err(|error| {
                RunServerError::RuntimeTimeout(ServerRuntimeRequestTimeoutError::from(error))
            })?;
    let router = server_runtime::RequestIdLayer.apply(
        server_runtime::HttpMetricsLayer::default().apply(
            server_runtime::SecurityHeadersLayer::from(server_runtime::ForwardedProtoTrust::Ignore)
                .with_content_security_policy(content_security_policy)
                .apply(
                    server_runtime::RequestTimeoutLayer::from(request_timeout)
                        .apply(server_runtime::AxumRouter::from(
                        axum::Router::new()
                            .nest(
                                str_constants::API_V1,
                                operational_routes.merge(api_routes.0).layer(
                                    axum::extract::DefaultBodyLimit::max(maximum_http_body_bytes),
                                ),
                            )
                            .merge(axum::Router::from(server_admin_frontend::routes()))
                            .merge(axum::Router::from(admin_html_routes))
                            .merge(admin_metrics_routes)
                            .merge(axum::Router::from(frontend_fallback_routes()))
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
                                                str_constants::ROUTE_VALIDATORS_COMMIT_HEADER_NAME,
                                            ),
                                            axum::http::HeaderName::from_static(
                                                str_constants::IDEMPOTENCY_KEY_ALT,
                                            ),
                                            axum::http::HeaderName::from_static(
                                                str_constants::IF_MATCH_ALT,
                                            ),
                                            axum::http::HeaderName::from_static(
                                                str_constants::X_CSRF_TOKEN_ALT,
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
                    )),
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
            RunServerError::AdminCleanupShutdown(ServerRuntimeBackgroundTaskShutdownError::from(
                error,
            ))
        })?;
    serve_result.map_err(|error| RunServerError::Serve(ServerRuntimeServeError::from(error)))?;
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
    let config = match server_config::Config::try_from_env() {
        Ok(config) => config,
        Err(config_error) => {
            let startup_error = RunServerError::Config(ServerConfigError::from(config_error));
            eprintln!("{startup_error}");
            return StdServerExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    initialization_tracing(config.tracing_format);
    match mk_runtime().and_then(|runtime| runtime.0.block_on(run_server(config))) {
        Ok(()) => StdServerExitCode::from(std::process::ExitCode::SUCCESS),
        Err(error) => {
            eprintln!("{error}");
            StdServerExitCode::from(std::process::ExitCode::FAILURE)
        }
    }
}
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn missing_page_redirects_to_default_authentication_page() {
        let response = tower::ServiceExt::oneshot(
            axum::Router::from(super::frontend_fallback_routes()),
            axum::http::Request::builder()
                .uri("/missing-page")
                .body(axum::body::Body::empty())
                .expect("cfe228d8"),
        )
        .await
        .expect("bd9f2b00");
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
        assert_eq!(str_constants::CONFIG_TRACING_INFO, "info");
    }
}
