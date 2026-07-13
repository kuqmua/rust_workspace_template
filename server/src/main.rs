const TRACING_DFLT_FILTER: &str = "info";
const CORS_ALLOW_ORIGIN_SPLIT_CH: char = ',';
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
impl std::fmt::Display for ServerRuntimeRequestTimeoutEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for ServerRuntimeRequestTimeoutEr {}
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
#[derive(Clone, Debug)]
struct AdminGeneratedAuthLayer {
    state: server_admin::auth::StdSharedAdminAuthSvcState,
}
#[derive(Clone, Debug)]
struct AdminGeneratedAuthService<Service> {
    inner: Service,
    state: server_admin::auth::StdSharedAdminAuthSvcState,
}
impl<Service> tower::Layer<Service> for AdminGeneratedAuthLayer {
    type Service = AdminGeneratedAuthService<Service>;
    fn layer(&self, inner: Service) -> Self::Service {
        AdminGeneratedAuthService {
            inner,
            state: self.state.clone(),
        }
    }
}
impl<Service> tower::Service<axum::extract::Request> for AdminGeneratedAuthService<Service>
where
    Service: tower::Service<axum::extract::Request, Response = axum::response::Response>
        + Clone
        + Send
        + 'static,
    Service::Future: Send + 'static,
    Service::Error: Send + 'static,
{
    type Error = Service::Error;
    type Future = std::pin::Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>,
    >;
    type Response = axum::response::Response;
    fn call(&mut self, req: axum::extract::Request) -> Self::Future {
        let mut inner = self.inner.clone();
        std::mem::swap(&mut inner, &mut self.inner);
        let state = self.state.clone();
        Box::pin(async move {
            let path = req.uri().path();
            let contract = server_admin::generated_tables::AdminRolesRouteContract::for_path(path)
                .map(|contract| (contract.permission(), contract.mutates()))
                .or_else(|| {
                    server_admin::generated_tables::AdminRolePermissionsRouteContract::for_path(
                        path,
                    )
                    .map(|contract| (contract.permission(), contract.mutates()))
                })
                .or_else(|| {
                    server_admin::generated_tables::AdminPermissionsRouteContract::for_path(path)
                        .map(|contract| (contract.permission(), contract.mutates()))
                })
                .or_else(|| {
                    server_admin::generated_tables::AdminSystemSettingsRouteContract::for_path(path)
                        .map(|contract| (contract.permission(), contract.mutates()))
                })
                .or_else(|| {
                    server_admin::generated_tables::AdminUserRolesRouteContract::for_path(path)
                        .map(|contract| (contract.permission(), contract.mutates()))
                })
                .or_else(|| {
                    server_admin::generated_tables::AdminUsersRouteContract::for_path(path)
                        .map(|contract| (contract.permission(), contract.mutates()))
                })
                .or_else(|| {
                    path.ends_with("/admin/openapi.json")
                        .then_some((Some("openapi:read"), false))
                })
                .or_else(|| {
                    path.ends_with("/admin/metrics")
                        .then_some((Some("metrics:read"), false))
                });
            let Some((Some(permission), mutates)) = contract else {
                return Ok(axum::response::IntoResponse::into_response(
                    server_admin::auth::AdminApiEr::Authorization,
                ));
            };
            if let Err(er) = server_admin::auth::authorize_generated_request(
                state.as_ref(),
                server_admin::HttpAdminHeaderMapRef::from(req.headers()),
                server_admin::StdAdminStrRef::from(permission),
                server_admin::StdAdminBool::from(mutates),
            )
            .await
            {
                return Ok(axum::response::IntoResponse::into_response(er));
            }
            tower::Service::call(&mut inner, req).await
        })
    }
    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        tower::Service::poll_ready(&mut self.inner, cx)
    }
}
#[derive(Clone, Copy)]
struct CorsAllowOriginTextRef<'text_lt>(&'text_lt str);
#[derive(Clone, Copy)]
struct CorsAllowOriginSplitCh(char);
#[derive(Clone, Copy)]
struct CorsAllowOriginSplitCount(usize);
struct AxumCorsAllowOriginHeaderValue(axum::http::HeaderValue);
struct AxumCorsAllowOriginHeaderValues(Vec<axum::http::HeaderValue>);
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
    #[error("invalid server runtime timeout: {0}")]
    RuntimeTimeout(ServerRuntimeRequestTimeoutEr),
    #[error("server failed: {0}")]
    Serve(ServerRuntimeServeEr),
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
        .route_layer(AdminGeneratedAuthLayer {
            state: generated_admin_auth_state,
        });
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
        pg_pool,
        config,
        project_git_info: &git_info::PROJECT_GIT_INFO,
    })
}
#[allow(clippy::single_call_fn)] // generic parser keeps separator handling reusable for non-header values and future config fields
fn parse_separated_values<T>(
    v: CorsAllowOriginTextRef<'_>,
    split_ch: CorsAllowOriginSplitCh,
    parse_value: impl FnMut(&str) -> Option<T>,
) -> Vec<T> {
    parse_separated_values_with_capacity(v, split_ch, parse_value)
}
#[allow(clippy::single_call_fn)] // extracted so separator-count capacity logic is reusable and testable
fn parse_separated_values_with_capacity<T>(
    v: CorsAllowOriginTextRef<'_>,
    split_ch: CorsAllowOriginSplitCh,
    parse_value: impl FnMut(&str) -> Option<T>,
) -> Vec<T> {
    let mut parsed = Vec::with_capacity(split_count(v, split_ch).0.saturating_add(1));
    parsed.extend(v.0.split(split_ch.0).filter_map(parse_value));
    parsed
}
#[allow(clippy::single_call_fn)] // isolated to keep capacity estimation reusable for parser helpers
fn split_count(
    v: CorsAllowOriginTextRef<'_>,
    split_ch: CorsAllowOriginSplitCh,
) -> CorsAllowOriginSplitCount {
    CorsAllowOriginSplitCount(
        v.0.chars()
            .filter(|checked_split_ch| checked_split_ch == &split_ch.0)
            .count(),
    )
}
#[allow(clippy::single_call_fn)] // extracted so per-value parse behavior can be reused and tested directly
fn parse_cors_allow_origin_value(
    value: CorsAllowOriginTextRef<'_>,
) -> Option<AxumCorsAllowOriginHeaderValue> {
    value
        .0
        .trim()
        .parse::<axum::http::HeaderValue>()
        .ok()
        .map(AxumCorsAllowOriginHeaderValue)
}
#[allow(clippy::single_call_fn)] // extracted for reuse in main setup and tests
fn parse_cors_allow_origin(v: CorsAllowOriginTextRef<'_>) -> AxumCorsAllowOriginHeaderValues {
    AxumCorsAllowOriginHeaderValues(parse_separated_values(
        v,
        CorsAllowOriginSplitCh(CORS_ALLOW_ORIGIN_SPLIT_CH),
        |value| {
            parse_cors_allow_origin_value(CorsAllowOriginTextRef(value))
                .map(|header_value| header_value.0)
        },
    ))
}
#[allow(clippy::single_call_fn)] // generic splitter is test-only and keeps separator behavior assertions deterministic
#[cfg(test)]
fn split_by_char(v: &str, split_ch: char) -> std::str::Split<'_, char> {
    v.split(split_ch)
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
    let tcp_listener = tokio::net::TcpListener::bind(
        config_lib::GetServiceSocketAddress::get_service_socket_address(&config),
    )
    .await
    .map_err(|er| RunServerEr::BindServiceSocket(StdServerIoEr(er)))?;
    let cors_origins = parse_cors_allow_origin(CorsAllowOriginTextRef(
        config_lib::GetCorsAllowOrigin::get_cors_allow_origin(&config),
    ));
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
                                        .allow_origin(cors_origins.0)
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
    server_runtime::serve_with_graceful_shutdown(
        server_runtime::TokioTcpListener::from(tcp_listener),
        router,
        async {
            if let Err(er) = tokio::signal::ctrl_c().await {
                eprintln!("failed to wait for ctrl-c signal: {er}");
            }
        },
        request_timeout,
    )
    .await
    .map_err(|er| RunServerEr::Serve(ServerRuntimeServeEr(er)))?;
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
    #[allow(clippy::single_call_fn)] // shared fixture keeps two-origin header expectations reusable across parser tests
    fn mk_two_origin_headers() -> Vec<axum::http::HeaderValue> {
        vec![
            axum::http::HeaderValue::from_static("https://a.example"),
            axum::http::HeaderValue::from_static("https://b.example"),
        ]
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps valid CORS header vector checks reusable across parser entry points
    fn assert_two_origin_headers(v: &[axum::http::HeaderValue]) {
        assert_eq!(v, mk_two_origin_headers());
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps split behavior checks concise across separator tests
    fn assert_split_by_char_parts(input: &str, split_ch: char, exp: &[&str]) {
        let parts = super::split_by_char(input, split_ch).collect::<Vec<_>>();
        assert_eq!(parts, exp);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps numeric parser checks consistent across separator helpers
    fn assert_parsed_u8_values(input: &str, split_ch: char, exp: &[u8]) {
        let parsed = super::parse_separated_values(
            super::CorsAllowOriginTextRef(input),
            super::CorsAllowOriginSplitCh(split_ch),
            |part| part.parse::<u8>().ok(),
        );
        assert_eq!(parsed, exp);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps parse helper outputs aligned across capacity and direct parser entry points
    fn assert_parsed_u8_values_with_capacity(input: &str, split_ch: char, exp: &[u8]) {
        let parsed = super::parse_separated_values_with_capacity(
            super::CorsAllowOriginTextRef(input),
            super::CorsAllowOriginSplitCh(split_ch),
            |part| part.parse::<u8>().ok(),
        );
        assert_eq!(parsed, exp);
    }
    #[test]
    fn parse_cors_allow_origin_keeps_valid_values() {
        let v = super::parse_cors_allow_origin(super::CorsAllowOriginTextRef(
            "https://a.example, https://b.example",
        ));
        assert_two_origin_headers(&v.0);
    }
    #[test]
    fn parse_cors_allow_origin_skips_invalid_values() {
        let v = super::parse_cors_allow_origin(super::CorsAllowOriginTextRef(
            "https://ok.example,bad\nvalue",
        ));
        assert_eq!(
            v.0,
            vec![axum::http::HeaderValue::from_static("https://ok.example")]
        );
    }
    #[test]
    fn parse_cors_allow_origin_keeps_empty_item_behavior() {
        let v = super::parse_cors_allow_origin(super::CorsAllowOriginTextRef(""));
        assert_eq!(v.0, vec![axum::http::HeaderValue::from_static("")]);
    }
    #[test]
    fn parse_cors_allow_origin_value_trims_and_parses_valid_header() {
        assert_eq!(
            super::parse_cors_allow_origin_value(super::CorsAllowOriginTextRef(
                " https://a.example "
            ))
            .map(|value| value.0),
            Some(axum::http::HeaderValue::from_static("https://a.example"))
        );
    }
    #[test]
    fn parse_cors_allow_origin_value_returns_none_for_invalid_header() {
        assert!(
            super::parse_cors_allow_origin_value(super::CorsAllowOriginTextRef("bad\nvalue"))
                .is_none()
        );
    }
    #[test]
    fn split_by_char_preserves_empty_segments_for_cors_separator() {
        assert_split_by_char_parts(
            "a,,b,",
            super::CORS_ALLOW_ORIGIN_SPLIT_CH,
            &["a", "", "b", ""],
        );
    }
    #[test]
    fn parse_cors_allow_origin_keeps_only_valid_values() {
        let parsed = super::parse_cors_allow_origin(super::CorsAllowOriginTextRef(
            "https://a.example,bad\nvalue, https://b.example",
        ));
        assert_two_origin_headers(&parsed.0);
    }
    #[test]
    fn parse_comma_separated_values_supports_non_header_parser() {
        assert_parsed_u8_values("1,2,nope,3", super::CORS_ALLOW_ORIGIN_SPLIT_CH, &[1, 2, 3]);
    }
    #[test]
    fn split_by_char_supports_custom_separator() {
        assert_split_by_char_parts("a;b;;", ';', &["a", "b", "", ""]);
    }
    #[test]
    fn parse_separated_values_supports_custom_separator() {
        assert_parsed_u8_values("10;20;bad;30", ';', &[10, 20, 30]);
    }
    #[test]
    fn parse_separated_values_with_capacity_supports_custom_separator() {
        assert_parsed_u8_values_with_capacity("10;20;bad;30", ';', &[10, 20, 30]);
    }
    #[test]
    fn split_count_returns_expected_separator_occurrences() {
        assert_eq!(
            super::split_count(
                super::CorsAllowOriginTextRef("a,b,,"),
                super::CorsAllowOriginSplitCh(',')
            )
            .0,
            3
        );
    }
    #[test]
    fn cors_allow_origin_split_char_is_stable() {
        assert_eq!(super::CORS_ALLOW_ORIGIN_SPLIT_CH, ',');
    }
    #[test]
    fn tracing_default_filter_is_stable() {
        assert_eq!(super::TRACING_DFLT_FILTER, "info");
    }
}
