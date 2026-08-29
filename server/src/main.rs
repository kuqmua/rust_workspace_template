pub mod admin_metrics_error;
pub mod axum_api_routes;
pub mod domain_types;
pub mod frontend_fallback_routes;
pub mod http_body_maximum_bytes;
pub mod make_postgresql_pool;
pub mod metrics_exporter_prometheus_build_error;
pub mod metrics_exporter_prometheus_renderer;
pub mod mount_service_routes;
pub mod run_server_error;
pub mod server_exit_code;
pub mod server_io_error;
pub mod shared_server_app_state_arc;
pub mod sqlx_server_pg_connect_error;
pub mod tokio_server_runtime;

fn main() -> server_exit_code::ServerExitCode {
    let config = match server_config::config::Config::try_from_env() {
        Ok(config) => config,
        Err(config_error) => {
            let startup_error = run_server_error::RunServerError::Config(config_error);
            tracing::error!(error = %startup_error, "server configuration failed");
            return server_exit_code::ServerExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    if let Err(error) = config.validate_for_startup() {
        tracing::error!(
            error = %run_server_error::RunServerError::ConfigProduction(error),
            "server production configuration validation failed"
        );
        return server_exit_code::ServerExitCode::from(std::process::ExitCode::FAILURE);
    }
    let tracing_format = if config.tracing_format == config_lib::tracing_format::TracingFormat::Json
    {
        server_observability::service_tracing_format::ServiceTracingFormat::Json
    } else {
        server_observability::service_tracing_format::ServiceTracingFormat::Text
    };
    let observability =
        match server_observability::init_service_observability::init_service_observability(
            tracing_format,
            server_observability::service_name::ServiceName::from(env!("CARGO_PKG_NAME")),
        ) {
            Ok(value) => value,
            Err(error) => {
                tracing::error!(
                    error = %run_server_error::RunServerError::ObservabilityInit(error),
                    "server observability initialization failed"
                );
                return server_exit_code::ServerExitCode::from(std::process::ExitCode::FAILURE);
            }
        };
    let run_result = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
        .map(tokio_server_runtime::TokioServerRuntime::from)
        .map_err(|error| {
            run_server_error::RunServerError::BuildRuntime(server_io_error::ServerIoError::from(error))
        })
        .and_then(|runtime| match config.svc_mode {
            config_lib::svc_mode::SvcMode::Migrate => {
                tokio::runtime::Runtime::from(runtime).block_on(async {
                    let pg_pool = make_postgresql_pool::make_postgresql_pool(&config).await?;
                    server_admin::prepare_postgresql::prepare_postgresql(app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(
                        pg_pool.as_ref(),
                    ))
                    .await
                    .map_err(run_server_error::RunServerError::PrepAdminPg)
                })
            }
            config_lib::svc_mode::SvcMode::Serve => {
                tokio::runtime::Runtime::from(runtime).block_on(async move {
                        let pg_pool = make_postgresql_pool::make_postgresql_pool(&config).await?;
                        let cleanup_batch_size = server_admin::admin_cleanup_batch_size::AdminCleanupBatchSize::try_from(1_000i64)
                            .map_err(run_server_error::RunServerError::AdminCleanupConfig)?;
                        let cleanup_retention = |seconds| {
                            server_admin::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds::try_from(seconds)
                                .map_err(run_server_error::RunServerError::AdminCleanupConfig)
                        };
                        let cleanup_cfg = server_admin::admin_cleanup_cfg::AdminCleanupCfg::new(
                            cleanup_batch_size,
                            cleanup_retention(604_800i64)?,
                            cleanup_retention(7_776_000i64)?,
                            cleanup_retention(86_400i64)?,
                            cleanup_retention(86_400i64)?,
                            cleanup_retention(3_600i64)?,
                        );
                        let cleanup_interval = server_runtime_http::run_interval_duration::RunIntervalDuration::try_from(
                            std::time::Duration::from_secs(300u64),
                        )
                        .map_err(run_server_error::RunServerError::RuntimeInterval)?;
                        let cleanup_pool = pg_pool.clone();
                        let Some(cleanup_task) = server_runtime_http::spawn_interval_task::spawn_interval_task(
                            Some(cleanup_interval),
                            move || {
                                let run_pool = cleanup_pool.clone();
                                async move {
                                    match server_admin::cleanup_admin_tables::cleanup_admin_tables(
                                        app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(run_pool.as_ref()),
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
                            return Err(run_server_error::RunServerError::RuntimeInterval(
                                server_runtime_http::std_run_interval_try_from_duration_error::StdRunIntervalTryFromDurationError,
                            ));
                        };
                        let tcp_listener = tokio::net::TcpListener::bind(
                            config_lib::domain_types::ServiceSocketAddressProvider::service_socket_address(&config),
                        )
                        .await
                        .map_err(|error| {
                            run_server_error::RunServerError::BindServiceSocket(
                                server_io_error::ServerIoError::from(error),
                            )
                        })?;
                        let actual_service_socket_address = tcp_listener.local_addr().map_err(|error| {
                            run_server_error::RunServerError::BindServiceSocket(
                                server_io_error::ServerIoError::from(error),
                            )
                        })?;
                        tracing::info!(frontend = %actual_service_socket_address);
                        let trusted_proxy_ranges = server_runtime_http::parse_trusted_proxy_ranges::parse_trusted_proxy_ranges(
                            server_runtime_http::trusted_proxy_ranges_text_ref::TrustedProxyRangesTextRef::from(
                                config.trusted_proxy_ranges_text.0.as_str(),
                            ),
                        )
                        .map_err(run_server_error::RunServerError::TrustedProxyRanges)?;
                        let cors_origins = Vec::<axum::http::HeaderValue>::from(
                            server_runtime_http::parse_cors_allow_origin::parse_cors_allow_origin(
                                server_runtime_http::http_cors_allow_origin_text_ref::HttpCorsAllowOriginTextRef::from(
                                    config_lib::domain_types::CorsAllowOriginProvider::cors_allow_origin(&config)
                                        .as_str(),
                                ),
                            )
                            .map_err(run_server_error::RunServerError::CorsAllowOrigin)?,
                        );
                        let admin_auth_state =
                            server_admin::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc::from_state(
                                server_admin::admin_auth_svc_state::AdminAuthSvcState::try_new(
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
                                .map_err(run_server_error::RunServerError::AdminAuthState)?,
                            );
                        let swagger_enabled = *config.admin_swagger_enabled;
                        let content_security_policy =
                            server_runtime_http::http_content_security_policy::HttpContentSecurityPolicy::try_from(
                                config.content_security_policy.as_ref().to_owned(),
                            )
                            .map_err(run_server_error::RunServerError::ContentSecurityPolicy)?;
                        let maximum_http_body_bytes =
                            *config_lib::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytesProvider::maximum_size_of_http_body_in_bytes(
                                &config,
                            );
                        let http_gzip_enabled = *config.http_gzip_enabled;
                        let request_timeout_seconds = config.request_timeout_seconds.get();
                        let app_state = shared_server_app_state_arc::SharedServerAppStateArc::from_state(
                            server_app_state::server_app_state::ServerAppState {
                                bulk_item_budget: server_runtime_core::resource_budget::ResourceBudget::new(
                                    server_runtime_core::resource_budget_maximum::ResourceBudgetMaximum::from(
                                        std::num::NonZeroUsize::new(4_096usize).unwrap_or(std::num::NonZeroUsize::MIN),
                                    ),
                                ),
                                config,
                                idempotency_response_budget: server_runtime_core::resource_budget::ResourceBudget::new(
                                    server_runtime_core::resource_budget_maximum::ResourceBudgetMaximum::from(
                                        std::num::NonZeroUsize::new(
                                            64usize.saturating_mul(constants_usize::VALUE_1_048_576),
                                        )
                                        .unwrap_or(std::num::NonZeroUsize::MIN),
                                    ),
                                ),
                                pg_pool,
                                project_git_info: git_info::project_git_info_value::project_git_info_value(),
                            },
                        );
                        let metrics_renderer = metrics_exporter_prometheus::PrometheusBuilder::new()
                            .install_recorder()
                            .map(metrics_exporter_prometheus_renderer::MetricsExporterPrometheusRenderer::from)
                            .map_err(|error| {
                                run_server_error::RunServerError::MetricsRecorder(
                                    metrics_exporter_prometheus_build_error::MetricsExporterPrometheusBuildError::from(error),
                                )
                            })?;
                        let admin_html_routes = server_admin::html_routes_with_swagger::html_routes_with_swagger(
                            admin_auth_state.clone(),
                            server_admin::admin_html_swagger_enabled::AdminHtmlSwaggerEnabled::from(swagger_enabled),
                        );
                        let html_metrics_renderer = metrics_renderer.clone();
                        let admin_metrics_routes = axum::Router::new()
                            .route(
                                server_admin_contract::admin_frontend_path::AdminFrontendPath::Metrics.get(),
                                axum::routing::get(async move || {
                                    server_runtime_http::metrics_response_body::MetricsResponseBody::try_from(
                                        metrics_exporter_prometheus::PrometheusHandle::from(html_metrics_renderer)
                                            .render(),
                                    )
                                    .map_or_else(
                                        |_error| {
                                            axum::response::IntoResponse::into_response(
                                                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                            )
                                        },
                                        |body| {
                                            let title_result =
                                                server_admin_frontend::admin_ssr_text::AdminSsrText::try_from(
                                                    constants_str::catalog::METRICS_ALT.to_owned(),
                                                );
                                            let text_result =
                                                server_admin_frontend::admin_ssr_text::AdminSsrText::try_from(
                                                    body.into_inner(),
                                                );
                                            match (title_result, text_result) {
                                                (Ok(title), Ok(text)) => axum::response::IntoResponse::into_response(
                                                    axum::response::Html(String::from(
                                                        server_admin_frontend::render_text_page::render_text_page(
                                                            server_admin_contract::admin_page::AdminPage::Metrics,
                                                            title,
                                                            text,
                                                        ),
                                                    )),
                                                ),
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
                            .route_layer(server_admin::admin_generated_auth_layer::AdminGeneratedAuthLayer::from(
                                admin_auth_state.clone(),
                            ));
                        let api_routes = {
                            let generated_admin_auth_state = admin_auth_state.clone();
                            let generated_table_logic_state: std::sync::Arc<
                                dyn pg_table::combination_of_app_state_logic_traits::CombinationOfAppStateLogicTraits,
                            > = std::sync::Arc::<server_app_state::server_app_state::ServerAppState<'static>>::clone(
                                app_state.get(),
                            );
                            let generated_table_state =
                                server_admin::shared_admin_generated_table_state_arc::SharedAdminGeneratedTableStateArc::from(
                                    generated_table_logic_state,
                                );
                            let generated_table_routes = axum::Router::from(
                                server_admin::generated_routes::generated_routes(&generated_table_state),
                            );
                            let open_api_contract = server_admin_contract::admin_route::AdminRoute::OpenApi.contract();
                            let documented_admin_routes = if *app_state.config.admin_swagger_enabled {
                                generated_table_routes.route(
                                    open_api_contract.path().as_ref(),
                                    axum::routing::on(
                                        axum::routing::MethodFilter::from(frontend_contract::to_axum_method_filter::to_axum_method_filter(
                                            open_api_contract.method(),
                                        )),
                                        async || {
                                            axum::Json(utoipa::openapi::OpenApi::from(
                                                server_admin::generated_open_api::generated_open_api(),
                                            ))
                                        },
                                    ),
                                )
                            } else {
                                generated_table_routes
                            }
                            .method_not_allowed_fallback(async || frontend_contract::api_problem_error::ApiProblemError::MethodNotAllowed);
                            let metrics_contract = server_admin_contract::admin_route::AdminRoute::Metrics.contract();
                            let secured_admin_routes = documented_admin_routes
                                .route(
                                    metrics_contract.path().as_ref(),
                                    axum::routing::on(
                                        axum::routing::MethodFilter::from(frontend_contract::to_axum_method_filter::to_axum_method_filter(
                                            metrics_contract.method(),
                                        )),
                                        async move || {
                                            server_runtime_http::metrics_response_body::MetricsResponseBody::try_from(
                                                metrics_exporter_prometheus::PrometheusHandle::from(metrics_renderer)
                                                    .render(),
                                            )
                                            .map(|body| {
                                                axum::response::IntoResponse::into_response((
                                                    axum::http::StatusCode::OK,
                                                    body.into_inner(),
                                                ))
                                            })
                                            .map_err(admin_metrics_error::AdminMetricsError::Render)
                                        },
                                    ),
                                )
                                .route_layer(server_admin::admin_generated_auth_layer::AdminGeneratedAuthLayer::from(
                                    generated_admin_auth_state,
                                ));
                            axum_api_routes::AxumApiRoutes::from(
                                axum::Router::new()
                                    .nest(
                                        server_admin_contract::admin_frontend_path::AdminFrontendPath::Root.get(),
                                        axum::Router::from(server_admin::admin_auth_routes::admin_auth_routes(
                                            admin_auth_state,
                                        )),
                                    )
                                    .nest(
                                        server_admin_contract::admin_frontend_path::AdminFrontendPath::Root.get(),
                                        secured_admin_routes,
                                    ),
                            )
                        };
                        let operational_routes = axum::Router::from(common_routes::common_routes::common_routes(
                            common_routes::arc_common_routes_app_state::ArcCommonRoutesAppState::from(std::sync::Arc::<
                                server_app_state::server_app_state::ServerAppState<'static>,
                            >::clone(app_state.get())),
                        ));
                        let request_timeout = server_runtime_http::request_timeout_duration::RequestTimeoutDuration::try_from(
                            std::time::Duration::from_secs(request_timeout_seconds),
                        )
                        .map_err(run_server_error::RunServerError::RuntimeTimeout)?;
                        let router = server_runtime_http::request_id_layer::RequestIdLayer::with_span_config(
                            server_runtime_http::http_request_span_config::HttpRequestSpanConfig::new(
                                server_observability::service_name::ServiceName::from(env!("CARGO_PKG_NAME")),
                                server_runtime_http::client_socket_addr::ClientSocketAddr::from(
                                    actual_service_socket_address,
                                ),
                                trusted_proxy_ranges,
                            ),
                        )
                        .apply(
                            server_runtime_http::http_metrics_layer::HttpMetricsLayer::default().apply(
                                server_runtime_http::security_headers_layer::SecurityHeadersLayer::from(
                                    server_runtime_http::forwarded_proto_trust::ForwardedProtoTrust::Ignore,
                                )
                                .with_content_security_policy(content_security_policy)
                                .apply(
                                    server_runtime_http::request_timeout_layer::RequestTimeoutLayer::from(request_timeout)
                                        .apply(server_runtime_http::axum_router::AxumRouter::from(
                                            axum::Router::from(mount_service_routes::mount_service_routes(
                                                server_runtime_http::axum_router::AxumRouter::from(operational_routes),
                                                api_routes,
                                                http_body_maximum_bytes::HttpBodyMaximumBytes::from(
                                                    maximum_http_body_bytes,
                                                ),
                                            ))
                                            .merge(axum::Router::from(
                                                server_admin_frontend::admin_frontend_routes::admin_frontend_routes(),
                                            ))
                                            .merge(axum::Router::from(admin_html_routes))
                                            .merge(admin_metrics_routes)
                                            .merge(axum::Router::from(
                                                frontend_fallback_routes::frontend_fallback_routes(),
                                            ))
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
                                                                constants_str::catalog::ROUTE_VALIDATORS_COMMIT_HEADER_NAME,
                                                            ),
                                                            axum::http::HeaderName::from_static(
                                                                constants_str::catalog::IDEMPOTENCY_KEY_ALT,
                                                            ),
                                                            axum::http::HeaderName::from_static(
                                                                constants_str::catalog::IF_MATCH_ALT,
                                                            ),
                                                            axum::http::HeaderName::from_static(
                                                                constants_str::catalog::X_CSRF_TOKEN_ALT,
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
                        let serve_result = server_runtime_http::serve_with_graceful_shutdown::serve_with_graceful_shutdown(
                            server_runtime_http::tokio_tcp_listener::TokioTcpListener::from(tcp_listener),
                            router,
                            async {
                                if let Err(error) =
                                    server_runtime_http::wait_for_service_shutdown_signal::wait_for_service_shutdown_signal().await
                                {
                                    tracing::error!(error = %error, "failed to wait for shutdown signal");
                                }
                            },
                            request_timeout,
                        )
                        .await;
                        let _cleanup_outcome = cleanup_task
                            .shutdown(request_timeout)
                            .await
                            .map_err(run_server_error::RunServerError::AdminCleanupShutdown)?;
                        serve_result.map_err(run_server_error::RunServerError::Serve)?;
                        Ok(())
                })
            }
        });
    if let Err(error) = run_result.as_ref() {
        tracing::error!(error = %error, "server terminated with an error");
    }
    let shutdown_result = observability
        .shutdown()
        .map_err(run_server_error::RunServerError::ObservabilityShutdown);
    match run_result.and(shutdown_result) {
        Ok(()) => server_exit_code::ServerExitCode::from(std::process::ExitCode::SUCCESS),
        Err(error) => {
            tracing::error!(error = %error, "server operation or observability shutdown failed");
            server_exit_code::ServerExitCode::from(std::process::ExitCode::FAILURE)
        }
    }
}
