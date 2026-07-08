const TRACING_DFLT_FILTER: &str = "info";
const CORS_ALLOW_ORIGIN_SPLIT_CH: char = ',';
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct ServerIoEr(std::io::Error);
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct ServerConfigEr(server_config::ConfigTryFromEnvEr);
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct ServerPgConnectEr(sqlx::Error);
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct ServerPrepPgEr(#[from] server_tbl_example::TblExamplePrepPgEr);
struct ApiRoutes(axum::Router);
#[derive(Clone, Copy)]
struct CorsAllowOriginTextRef<'text_lt>(&'text_lt str);
#[derive(Clone, Copy)]
struct CorsAllowOriginSplitCh(char);
#[derive(Clone, Copy)]
struct CorsAllowOriginSplitCount(usize);
struct CorsAllowOriginHeaderValue(axum::http::HeaderValue);
struct CorsAllowOriginHeaderValues(Vec<axum::http::HeaderValue>);
struct ServerRuntime(tokio::runtime::Runtime);
struct ServerExitCode(std::process::ExitCode);
impl std::process::Termination for ServerExitCode {
    fn report(self) -> std::process::ExitCode {
        self.0
    }
}
#[derive(Debug, thiserror::Error)]
enum RunServerEr {
    #[error("failed to bind service socket: {0}")]
    BindServiceSocket(ServerIoEr),
    #[error("failed to build tokio runtime: {0}")]
    BuildRuntime(ServerIoEr),
    #[error("failed to read configuration from environment: {0}")]
    Config(ServerConfigEr),
    #[error("failed to build governor config")]
    GovernorConfig,
    #[error("failed to connect to postgres: {0}")]
    PgConnect(ServerPgConnectEr),
    #[error("failed to prepare postgres schema: {0}")]
    PrepPg(ServerPrepPgEr),
    #[error("server failed: {0}")]
    Serve(ServerIoEr),
}
#[allow(clippy::single_call_fn)] // route wiring is reused by startup flow and isolated from layer setup
fn mk_api_routes(
    app_state: &std::sync::Arc<server_app_state::ServerAppState<'static>>,
) -> ApiRoutes {
    ApiRoutes(
        axum::Router::new()
            .merge(
                cmn_routes::cmn_routes(cmn_routes::CmnRoutesAppState::from(std::sync::Arc::<
                    server_app_state::ServerAppState<'static>,
                >::clone(
                    app_state
                )))
                .0,
            )
            .merge(server_tbl_example::TblExample::routes(std::sync::Arc::<
                server_app_state::ServerAppState<'static>,
            >::clone(
                app_state
            ))),
    )
}
#[allow(clippy::single_call_fn)] // keeps state creation shape reusable and type-stable in one place
fn mk_app_state(
    config: server_config::Config,
    pg_pool: sqlx::PgPool,
) -> std::sync::Arc<server_app_state::ServerAppState<'static>> {
    std::sync::Arc::new(server_app_state::ServerAppState {
        pg_pool: app_state::PgPool(pg_pool),
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
) -> Option<CorsAllowOriginHeaderValue> {
    value
        .0
        .trim()
        .parse::<axum::http::HeaderValue>()
        .ok()
        .map(CorsAllowOriginHeaderValue)
}
#[allow(clippy::single_call_fn)] // extracted for reuse in main setup and tests
fn parse_cors_allow_origin(v: CorsAllowOriginTextRef<'_>) -> CorsAllowOriginHeaderValues {
    CorsAllowOriginHeaderValues(parse_separated_values(
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
fn mk_runtime() -> Result<ServerRuntime, RunServerEr> {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
        .map(ServerRuntime)
        .map_err(|er| RunServerEr::BuildRuntime(ServerIoEr(er)))
}
#[allow(clippy::single_call_fn)] // isolated pool builder keeps startup flow linear and reuses config getters in one place
async fn mk_pg_pool(config: &server_config::Config) -> Result<sqlx::PgPool, RunServerEr> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(*app_state::GetPgPoolMaxConnections::get_pg_pool_max_connections(config))
        .connect(secrecy::ExposeSecret::expose_secret(
            app_state::GetDatabaseUrl::get_database_url(config),
        ))
        .await
        .map_err(|er| RunServerEr::PgConnect(ServerPgConnectEr(er)))
}
#[allow(clippy::single_call_fn)] // startup flow is grouped for separation from process/bootstrap concerns
async fn run_server() -> Result<(), RunServerEr> {
    let config = server_config::Config::try_from_env()
        .map_err(|er| RunServerEr::Config(ServerConfigEr(er)))?;
    let pg_pool = mk_pg_pool(&config).await?;
    server_tbl_example::TblExample::prep_pg(&pg_pool)
        .await
        .map_err(|er| RunServerEr::PrepPg(ServerPrepPgEr::from(er)))?;
    let tcp_listener = tokio::net::TcpListener::bind(
        app_state::GetServiceSocketAddress::get_service_socket_address(&config),
    )
    .await
    .map_err(|er| RunServerEr::BindServiceSocket(ServerIoEr(er)))?;
    let cors_origins = parse_cors_allow_origin(CorsAllowOriginTextRef(
        app_state::GetCorsAllowOrigin::get_cors_allow_origin(&config),
    ));
    let app_state = mk_app_state(config, pg_pool);
    let api_routes = mk_api_routes(&app_state);
    let governor_conf = std::sync::Arc::new(
        tower_governor::governor::GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(10)
            .finish()
            .ok_or(RunServerEr::GovernorConfig)?,
    );
    axum::serve(
        tcp_listener,
        axum::Router::new()
            .nest("/api/v1", api_routes.0)
            .layer(
                tower::ServiceBuilder::new()
                    .layer(tower_http::request_id::PropagateRequestIdLayer::x_request_id())
                    .layer(tower_http::trace::TraceLayer::new_for_http())
                    .layer(tower_http::request_id::SetRequestIdLayer::x_request_id(
                        tower_http::request_id::MakeRequestUuid,
                    ))
                    .layer(tower_http::cors::CorsLayer::new().allow_origin(cors_origins.0))
                    .layer(tower_governor::GovernorLayer::new(governor_conf)),
            )
            .into_make_service(),
    )
    .with_graceful_shutdown(async {
        if let Err(er) = tokio::signal::ctrl_c().await {
            eprintln!("failed to wait for ctrl-c signal: {er}");
        }
    })
    .await
    .map_err(|er| RunServerEr::Serve(ServerIoEr(er)))?;
    Ok(())
}
fn main() -> ServerExitCode {
    init_tracing();
    match mk_runtime().and_then(|runtime| runtime.0.block_on(run_server())) {
        Ok(()) => ServerExitCode(std::process::ExitCode::SUCCESS),
        Err(er) => {
            eprintln!("{er}");
            ServerExitCode(std::process::ExitCode::FAILURE)
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
