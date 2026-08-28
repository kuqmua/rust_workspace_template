#[allow(clippy::single_call_fn)] // named command or composition stage has one orchestration owner
pub(super) async fn run_server(
    config: server_config::domain_types::Config,
) -> Result<(), crate::domain_types::RunServerError> {
    let pg_pool = crate::make_postgresql_pool::make_postgresql_pool(&config).await?;
    let cleanup_cfg = crate::configuration::configuration()?;
    let cleanup_interval = crate::interval::interval()?;
    let cleanup_pool = pg_pool.clone();
    let Some(cleanup_task) = server_runtime_http::domain_types::spawn_interval_task(
        Some(cleanup_interval),
        move || {
            let run_pool = cleanup_pool.clone();
            async move {
                match server_admin::domain_types::cleanup_admin_tables(
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
        return Err(crate::domain_types::RunServerError::RuntimeInterval(
            crate::domain_types::ServerRuntimeRunIntervalError::from(
                server_runtime_http::domain_types::StdRunIntervalTryFromDurationError,
            ),
        ));
    };
    let tcp_listener = tokio::net::TcpListener::bind(
        config_lib::domain_types::ServiceSocketAddressProvider::service_socket_address(&config),
    )
    .await
    .map_err(|error| {
        crate::domain_types::RunServerError::BindServiceSocket(
            crate::domain_types::ServerIoError::from(error),
        )
    })?;
    let actual_service_socket_address = tcp_listener.local_addr().map_err(|error| {
        crate::domain_types::RunServerError::BindServiceSocket(
            crate::domain_types::ServerIoError::from(error),
        )
    })?;
    tracing::info!(frontend = %actual_service_socket_address);
    let trusted_proxy_ranges = server_runtime_http::domain_types::parse_trusted_proxy_ranges(
        server_runtime_http::domain_types::TrustedProxyRangesTextRef::from(
            config.trusted_proxy_ranges_text.0.as_str(),
        ),
    )
    .map_err(|error| {
        crate::domain_types::RunServerError::TrustedProxyRanges(
            crate::domain_types::ServerRuntimeTrustedProxyRangesParseError::from(error),
        )
    })?;
    let cors_origins = Vec::<axum::http::HeaderValue>::from(
        server_runtime_http::domain_types::parse_cors_allow_origin(
            server_runtime_http::domain_types::HttpCorsAllowOriginTextRef::from(
                config_lib::domain_types::CorsAllowOriginProvider::cors_allow_origin(&config)
                    .as_str(),
            ),
        )
        .map_err(crate::domain_types::RunServerError::CorsAllowOrigin)?,
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
            .map_err(|error| {
                crate::domain_types::RunServerError::AdminAuthState(
                    crate::domain_types::ServerAdminAuthSvcStateBuildError::from(error),
                )
            })?,
        ));
    let swagger_enabled = *config.admin_swagger_enabled;
    let content_security_policy =
        server_runtime_http::domain_types::HttpContentSecurityPolicy::try_from(
            config.content_security_policy.as_ref().to_owned(),
        )
        .map_err(|error| {
            crate::domain_types::RunServerError::ContentSecurityPolicy(
                crate::domain_types::ServerRuntimeContentSecurityPolicyError::from(error),
            )
        })?;
    let maximum_http_body_bytes =
        *config_lib::domain_types::MaximumSizeOfHttpBodyInBytesProvider::maximum_size_of_http_body_in_bytes(
            &config,
        );
    let http_gzip_enabled = *config.http_gzip_enabled;
    let request_timeout_seconds = config.request_timeout_seconds.get();
    let app_state = crate::domain_types::SharedServerAppStateArc::from(std::sync::Arc::new(
        server_app_state::domain_types::ServerAppState {
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
            project_git_info: git_info::domain_types::project_git_info_value(),
        },
    ));
    let metrics_renderer = metrics_exporter_prometheus::PrometheusBuilder::new()
        .install_recorder()
        .map(crate::domain_types::MetricsExporterPrometheusRenderer::from)
        .map_err(|error| {
            crate::domain_types::RunServerError::MetricsRecorder(
                crate::domain_types::MetricsExporterPrometheusBuildError::from(error),
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
    let api_routes =
        crate::routes::build_server_routes(admin_auth_state, &app_state, metrics_renderer);
    let operational_routes = axum::Router::from(common_routes::adapters::common_routes(
        common_routes::domain_types::ArcCommonRoutesAppState::from(std::sync::Arc::<
            server_app_state::domain_types::ServerAppState<'static>,
        >::clone(
            app_state.get()
        )),
    ));
    let request_timeout = server_runtime_http::domain_types::RequestTimeoutDuration::try_from(
        std::time::Duration::from_secs(request_timeout_seconds),
    )
    .map_err(|error| {
        crate::domain_types::RunServerError::RuntimeTimeout(
            crate::domain_types::ServerRuntimeRequestTimeoutError::from(error),
        )
    })?;
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
                        axum::Router::from(crate::mount_service_routes::mount_service_routes(
                            server_runtime_http::domain_types::AxumRouter::from(operational_routes),
                            api_routes,
                            crate::domain_types::HttpBodyMaximumBytes::from(
                                maximum_http_body_bytes,
                            ),
                        ))
                        .merge(axum::Router::from(
                            server_admin_frontend::domain_types::admin_frontend_routes(),
                        ))
                        .merge(axum::Router::from(admin_html_routes))
                        .merge(admin_metrics_routes)
                        .merge(axum::Router::from(
                            crate::frontend_fallback_routes::frontend_fallback_routes(),
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
        .map_err(|error| {
            crate::domain_types::RunServerError::AdminCleanupShutdown(
                crate::domain_types::ServerRuntimeBackgroundTaskShutdownError::from(error),
            )
        })?;
    serve_result.map_err(|error| {
        crate::domain_types::RunServerError::Serve(
            crate::domain_types::ServerRuntimeServeError::from(error),
        )
    })?;
    Ok(())
}
