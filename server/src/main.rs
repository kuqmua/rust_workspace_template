mod admin_metrics_error;
mod axum_api_routes;
mod domain_types;
mod frontend_fallback_routes;
mod http_body_maximum_bytes;
mod make_postgresql_pool;
mod metrics_exporter_prometheus_build_error;
mod metrics_exporter_prometheus_renderer;
mod mount_service_routes;
mod run_server_error;
mod server_exit_code;
mod server_io_error;
mod shared_server_app_state_arc;
mod sqlx_server_pg_connect_error;
mod tokio_server_runtime;

fn main() -> domain_types::ServerExitCode {
    let config = match server_config::config::Config::try_from_env() {
        Ok(config) => config,
        Err(config_error) => {
            let startup_error = domain_types::RunServerError::Config(config_error);
            tracing::error!(error = %startup_error, "server configuration failed");
            return domain_types::ServerExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    if let Err(error) = config.validate_for_startup() {
        tracing::error!(
            error = %domain_types::RunServerError::ConfigProduction(error),
            "server production configuration validation failed"
        );
        return domain_types::ServerExitCode::from(std::process::ExitCode::FAILURE);
    }
    let tracing_format =
        if config.tracing_format == config_lib::domain_types::types::TracingFormat::Json {
            server_runtime_http::domain_types::ServiceTracingFormat::Json
        } else {
            server_runtime_http::domain_types::ServiceTracingFormat::Text
        };
    let observability = match server_runtime_http::domain_types::init_service_observability(
        tracing_format,
        server_runtime_http::domain_types::ServiceName::from(env!("CARGO_PKG_NAME")),
    ) {
        Ok(value) => value,
        Err(error) => {
            tracing::error!(
                error = %domain_types::RunServerError::ObservabilityInit(error),
                "server observability initialization failed"
            );
            return domain_types::ServerExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    let run_result = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
        .map(domain_types::TokioServerRuntime::from)
        .map_err(|error| {
            domain_types::RunServerError::BuildRuntime(domain_types::ServerIoError::from(error))
        })
        .and_then(|runtime| match config.svc_mode {
            config_lib::domain_types::types::SvcMode::Migrate => {
                tokio::runtime::Runtime::from(runtime).block_on(async {
                    let pg_pool = make_postgresql_pool::make_postgresql_pool(&config).await?;
                    server_admin::domain_types::prepare_postgresql(app_state::SqlxPgPoolRef::from(
                        pg_pool.as_ref(),
                    ))
                    .await
                    .map_err(domain_types::RunServerError::PrepAdminPg)
                })
            }
            config_lib::domain_types::types::SvcMode::Serve => {
                tokio::runtime::Runtime::from(runtime).block_on(async move {
                        let pg_pool = make_postgresql_pool::make_postgresql_pool(&config).await?;
                        let cleanup_batch_size = server_admin::domain_types::AdminCleanupBatchSize::try_from(1_000i64)
                            .map_err(domain_types::RunServerError::AdminCleanupConfig)?;
                        let cleanup_retention = |seconds| {
                            server_admin::domain_types::AdminCleanupRetentionSeconds::try_from(seconds)
                                .map_err(domain_types::RunServerError::AdminCleanupConfig)
                        };
                        let cleanup_cfg = server_admin::domain_types::AdminCleanupCfg::new(
                            cleanup_batch_size,
                            cleanup_retention(604_800i64)?,
                            cleanup_retention(7_776_000i64)?,
                            cleanup_retention(86_400i64)?,
                            cleanup_retention(86_400i64)?,
                            cleanup_retention(3_600i64)?,
                        );
                        let cleanup_interval = server_runtime_http::domain_types::RunIntervalDuration::try_from(
                            std::time::Duration::from_secs(300u64),
                        )
                        .map_err(domain_types::RunServerError::RuntimeInterval)?;
                        let cleanup_pool = pg_pool.clone();
                        let Some(cleanup_task) = server_runtime_http::domain_types::spawn_interval_task(
                            Some(cleanup_interval),
                            move || {
                                let run_pool = cleanup_pool.clone();
                                async move {
                                    match server_admin::domain_types::cleanup_admin_tables(
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
                            return Err(domain_types::RunServerError::RuntimeInterval(
                                server_runtime_http::domain_types::StdRunIntervalTryFromDurationError,
                            ));
                        };
                        let tcp_listener = tokio::net::TcpListener::bind(
                            config_lib::domain_types::ServiceSocketAddressProvider::service_socket_address(&config),
                        )
                        .await
                        .map_err(|error| {
                            domain_types::RunServerError::BindServiceSocket(
                                domain_types::ServerIoError::from(error),
                            )
                        })?;
                        let actual_service_socket_address = tcp_listener.local_addr().map_err(|error| {
                            domain_types::RunServerError::BindServiceSocket(
                                domain_types::ServerIoError::from(error),
                            )
                        })?;
                        tracing::info!(frontend = %actual_service_socket_address);
                        let trusted_proxy_ranges = server_runtime_http::domain_types::parse_trusted_proxy_ranges(
                            server_runtime_http::domain_types::TrustedProxyRangesTextRef::from(
                                config.trusted_proxy_ranges_text.0.as_str(),
                            ),
                        )
                        .map_err(domain_types::RunServerError::TrustedProxyRanges)?;
                        let cors_origins = Vec::<axum::http::HeaderValue>::from(
                            server_runtime_http::domain_types::parse_cors_allow_origin(
                                server_runtime_http::domain_types::HttpCorsAllowOriginTextRef::from(
                                    config_lib::domain_types::CorsAllowOriginProvider::cors_allow_origin(&config)
                                        .as_str(),
                                ),
                            )
                            .map_err(domain_types::RunServerError::CorsAllowOrigin)?,
                        );
                        let admin_auth_state =
                            server_admin::domain_types::auth::SharedAdminAuthSvcStateArc::from(std::sync::Arc::new(
                                server_admin::domain_types::auth::AdminAuthSvcState::try_new(
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
                                .map_err(domain_types::RunServerError::AdminAuthState)?,
                            ));
                        let swagger_enabled = *config.admin_swagger_enabled;
                        let content_security_policy =
                            server_runtime_http::domain_types::HttpContentSecurityPolicy::try_from(
                                config.content_security_policy.as_ref().to_owned(),
                            )
                            .map_err(domain_types::RunServerError::ContentSecurityPolicy)?;
                        let maximum_http_body_bytes =
                            *config_lib::domain_types::MaximumSizeOfHttpBodyInBytesProvider::maximum_size_of_http_body_in_bytes(
                                &config,
                            );
                        let http_gzip_enabled = *config.http_gzip_enabled;
                        let request_timeout_seconds = config.request_timeout_seconds.get();
                        let app_state = domain_types::SharedServerAppStateArc::from(std::sync::Arc::new(
                            server_app_state::ServerAppState {
                                bulk_item_budget: server_runtime_http::domain_types::ResourceBudget::new(
                                    server_runtime_http::domain_types::ResourceBudgetMaximum::from(
                                        std::num::NonZeroUsize::new(4_096usize).unwrap_or(std::num::NonZeroUsize::MIN),
                                    ),
                                ),
                                config,
                                idempotency_response_budget: server_runtime_http::domain_types::ResourceBudget::new(
                                    server_runtime_http::domain_types::ResourceBudgetMaximum::from(
                                        std::num::NonZeroUsize::new(
                                            64usize.saturating_mul(constants_usize::VALUE_1_048_576),
                                        )
                                        .unwrap_or(std::num::NonZeroUsize::MIN),
                                    ),
                                ),
                                pg_pool,
                                project_git_info: git_info::project_git_info_value(),
                            },
                        ));
                        let metrics_renderer = metrics_exporter_prometheus::PrometheusBuilder::new()
                            .install_recorder()
                            .map(domain_types::MetricsExporterPrometheusRenderer::from)
                            .map_err(|error| {
                                domain_types::RunServerError::MetricsRecorder(
                                    domain_types::MetricsExporterPrometheusBuildError::from(error),
                                )
                            })?;
                        let admin_html_routes = server_admin::domain_types::auth::html_routes_with_swagger(
                            admin_auth_state.clone(),
                            server_admin::domain_types::auth::AdminHtmlSwaggerEnabled::from(swagger_enabled),
                        );
                        let html_metrics_renderer = metrics_renderer.clone();
                        let admin_metrics_routes = axum::Router::new()
                            .route(
                                server_admin_contract::domain_types::AdminFrontendPath::Metrics.get(),
                                axum::routing::get(async move || {
                                    server_runtime_http::domain_types::MetricsResponseBody::try_from(
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
                                                server_admin_frontend::domain_types::ssr::AdminSsrText::try_from(
                                                    constants_str::METRICS_ALT.to_owned(),
                                                );
                                            let text_result =
                                                server_admin_frontend::domain_types::ssr::AdminSsrText::try_from(
                                                    body.into_inner(),
                                                );
                                            match (title_result, text_result) {
                                                (Ok(title), Ok(text)) => axum::response::IntoResponse::into_response(
                                                    axum::response::Html(String::from(
                                                        server_admin_frontend::domain_types::ssr::render_text_page(
                                                            server_admin_contract::domain_types::AdminPage::Metrics,
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
                            .route_layer(server_admin::domain_types::AdminGeneratedAuthLayer::from(
                                admin_auth_state.clone(),
                            ));
                        let api_routes = {
                            let generated_admin_auth_state = admin_auth_state.clone();
                            let generated_table_logic_state: std::sync::Arc<
                                dyn server_admin::domain_types::CombinationOfAppStateLogicTraits,
                            > = std::sync::Arc::<server_app_state::ServerAppState<'static>>::clone(
                                app_state.get(),
                            );
                            let generated_table_state =
                                server_admin::domain_types::generated_tables::SharedAdminGeneratedTableStateArc::from(
                                    generated_table_logic_state,
                                );
                            let generated_table_routes = axum::Router::from(
                                server_admin::domain_types::generated_tables::generated_routes(&generated_table_state),
                            );
                            let open_api_contract = server_admin_contract::domain_types::AdminRoute::OpenApi.contract();
                            let documented_admin_routes = if *app_state.config.admin_swagger_enabled {
                                generated_table_routes.route(
                                    open_api_contract.path().as_ref(),
                                    axum::routing::on(
                                        axum::routing::MethodFilter::from(frontend_contract::to_axum_method_filter(
                                            open_api_contract.method(),
                                        )),
                                        async || {
                                            axum::Json(utoipa::openapi::OpenApi::from(
                                                server_admin::domain_types::generated_tables::generated_open_api(),
                                            ))
                                        },
                                    ),
                                )
                            } else {
                                generated_table_routes
                            }
                            .method_not_allowed_fallback(async || frontend_contract::ApiProblemError::MethodNotAllowed);
                            let metrics_contract = server_admin_contract::domain_types::AdminRoute::Metrics.contract();
                            let secured_admin_routes = documented_admin_routes
                                .route(
                                    metrics_contract.path().as_ref(),
                                    axum::routing::on(
                                        axum::routing::MethodFilter::from(frontend_contract::to_axum_method_filter(
                                            metrics_contract.method(),
                                        )),
                                        async move || {
                                            server_runtime_http::domain_types::MetricsResponseBody::try_from(
                                                metrics_exporter_prometheus::PrometheusHandle::from(metrics_renderer)
                                                    .render(),
                                            )
                                            .map(|body| {
                                                axum::response::IntoResponse::into_response((
                                                    axum::http::StatusCode::OK,
                                                    body.into_inner(),
                                                ))
                                            })
                                            .map_err(domain_types::AdminMetricsError::Render)
                                        },
                                    ),
                                )
                                .route_layer(server_admin::domain_types::AdminGeneratedAuthLayer::from(
                                    generated_admin_auth_state,
                                ));
                            domain_types::AxumApiRoutes::from(
                                axum::Router::new()
                                    .nest(
                                        server_admin_contract::domain_types::AdminFrontendPath::Root.get(),
                                        axum::Router::from(server_admin::domain_types::auth::admin_auth_routes(
                                            admin_auth_state,
                                        )),
                                    )
                                    .nest(
                                        server_admin_contract::domain_types::AdminFrontendPath::Root.get(),
                                        secured_admin_routes,
                                    ),
                            )
                        };
                        let operational_routes = axum::Router::from(common_routes::common_routes(
                            common_routes::ArcCommonRoutesAppState::from(std::sync::Arc::<
                                server_app_state::ServerAppState<'static>,
                            >::clone(app_state.get())),
                        ));
                        let request_timeout = server_runtime_http::domain_types::RequestTimeoutDuration::try_from(
                            std::time::Duration::from_secs(request_timeout_seconds),
                        )
                        .map_err(domain_types::RunServerError::RuntimeTimeout)?;
                        let router = server_runtime_http::domain_types::RequestIdLayer::with_span_config(
                            server_runtime_http::domain_types::HttpRequestSpanConfig::new(
                                server_runtime_http::domain_types::ServiceName::from(env!("CARGO_PKG_NAME")),
                                server_runtime_http::domain_types::ClientSocketAddr::from(
                                    actual_service_socket_address,
                                ),
                                trusted_proxy_ranges,
                            ),
                        )
                        .apply(
                            server_runtime_http::domain_types::HttpMetricsLayer::default().apply(
                                server_runtime_http::domain_types::SecurityHeadersLayer::from(
                                    server_runtime_http::domain_types::ForwardedProtoTrust::Ignore,
                                )
                                .with_content_security_policy(content_security_policy)
                                .apply(
                                    server_runtime_http::domain_types::RequestTimeoutLayer::from(request_timeout)
                                        .apply(server_runtime_http::domain_types::AxumRouter::from(
                                            axum::Router::from(mount_service_routes::mount_service_routes(
                                                server_runtime_http::domain_types::AxumRouter::from(operational_routes),
                                                api_routes,
                                                domain_types::HttpBodyMaximumBytes::from(
                                                    maximum_http_body_bytes,
                                                ),
                                            ))
                                            .merge(axum::Router::from(
                                                server_admin_frontend::domain_types::admin_frontend_routes(),
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
                                        )),
                                ),
                            ),
                        );
                        let serve_result = server_runtime_http::domain_types::serve_with_graceful_shutdown(
                            server_runtime_http::domain_types::TokioTcpListener::from(tcp_listener),
                            router,
                            async {
                                if let Err(error) =
                                    server_runtime_http::domain_types::wait_for_service_shutdown_signal().await
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
                            .map_err(domain_types::RunServerError::AdminCleanupShutdown)?;
                        serve_result.map_err(domain_types::RunServerError::Serve)?;
                        Ok(())
                })
            }
        });
    if let Err(error) = run_result.as_ref() {
        tracing::error!(error = %error, "server terminated with an error");
    }
    let shutdown_result = observability
        .shutdown()
        .map_err(domain_types::RunServerError::ObservabilityShutdown);
    match run_result.and(shutdown_result) {
        Ok(()) => domain_types::ServerExitCode::from(std::process::ExitCode::SUCCESS),
        Err(error) => {
            tracing::error!(error = %error, "server operation or observability shutdown failed");
            domain_types::ServerExitCode::from(std::process::ExitCode::FAILURE)
        }
    }
}
