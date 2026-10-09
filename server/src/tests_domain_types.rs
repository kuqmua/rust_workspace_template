#[cfg(test)]
mod tests {
    #[test]
    fn test_server_exit_code_report_preserves_every_byte_exit_status() {
        assert!((u8::MIN..=u8::MAX).all(|value| {
            std::process::Termination::report(crate::server_exit_code::ServerExitCode::from(
                std::process::ExitCode::from(value),
            )) == std::process::ExitCode::from(value)
        }));
    }

    #[test]
    fn test_server_startup_errors_preserve_operation_context_and_inner_messages() {
        let server_io_error = crate::server_io_error::ServerIoError::from(std::io::Error::from(
            std::io::ErrorKind::PermissionDenied,
        ));
        let io_message = server_io_error.to_string();
        let sqlx_server_pg_connect_error =
            crate::sqlx_server_pg_connect_error::SqlxServerPgConnectError::from(
                sqlx::Error::RowNotFound,
            );
        let database_message = sqlx_server_pg_connect_error.to_string();
        let std_run_interval_try_from_duration_error =
            server_runtime_http::std_run_interval_try_from_duration_error::StdRunIntervalTryFromDurationError::Zero;
        let interval_message = std_run_interval_try_from_duration_error.to_string();
        let std_request_timeout_try_from_duration_error =
            server_runtime_http::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero;
        let timeout_message = std_request_timeout_try_from_duration_error.to_string();
        assert!(
            [
                (
                    crate::run_server_error::RunServerError::BuildRuntime(server_io_error),
                    stringify!(failed to build tokio runtime),
                    io_message,
                ),
                (
                    crate::run_server_error::RunServerError::PgConnect(
                        sqlx_server_pg_connect_error
                    ),
                    stringify!(failed to connect to postgres),
                    database_message,
                ),
                (
                    crate::run_server_error::RunServerError::RuntimeInterval(
                        std_run_interval_try_from_duration_error,
                    ),
                    stringify!(invalid server runtime interval),
                    interval_message,
                ),
                (
                    crate::run_server_error::RunServerError::RuntimeTimeout(
                        std_request_timeout_try_from_duration_error,
                    ),
                    stringify!(invalid server runtime timeout),
                    timeout_message,
                ),
            ]
            .into_iter()
            .all(|(error, context, source_message)| error.to_string()
                == context
                    .chars()
                    .chain([':', ' '])
                    .chain(source_message.chars())
                    .collect::<String>())
        );
        assert_eq!(
            crate::run_server_error::RunServerError::PgPoolConfiguration.to_string(),
            stringify!(postgres minimum connections must not exceed maximum connections)
        );
    }

    #[test]
    fn test_bind_service_socket_error_reports_address_and_preserves_source() {
        assert!([
            std::net::SocketAddr::from((std::net::Ipv4Addr::LOCALHOST, 8080u16)),
            std::net::SocketAddr::from((std::net::Ipv6Addr::LOCALHOST, 8081u16)),
        ]
        .into_iter()
        .all(|socket_addr| {
            let service_socket_address =
                <config_lib::domain_types::ServiceSocketAddress as config_lib::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                    config_lib::std_env_var_ok::StdEnvVarOk::try_from(socket_addr.to_string())
                        .unwrap_or_else(config_lib::std_env_var_ok::StdEnvVarOk::from),
                )
                .expect(constants_str::DIAGNOSTIC_572E4C7E);
            let server_io_error = crate::server_io_error::ServerIoError::from(
                std::io::Error::from(std::io::ErrorKind::AddrInUse),
            );
            let source_message = server_io_error.to_string();
            let run_server_error = crate::run_server_error::RunServerError::BindServiceSocket(
                server_io_error,
                service_socket_address,
            );
            let message = run_server_error.to_string();
            message.starts_with(constants_str::SERVICE_SOCKET_BIND_FAILED)
                && message.contains(&socket_addr.to_string())
                && message.ends_with(&source_message)
                && std::error::Error::source(&run_server_error).is_some_and(|error| {
                    error.is::<crate::server_io_error::ServerIoError>()
                        && error.to_string() == source_message
                })
        }));
    }

    #[tokio::test]
    async fn test_administrator_asset_route_preserves_static_file_serving() {
        let router = axum::Router::from(
            server_runtime_http::security_headers_layer::SecurityHeadersLayer::from(
                server_runtime_http::forwarded_proto_trust::ForwardedProtoTrust::Ignore,
            )
            .apply(server_runtime_http::axum_router::AxumRouter::from(
                axum::Router::from(frontend_admin::admin_frontend_routes::admin_frontend_routes()),
            )),
        );
        let response = tower::ServiceExt::oneshot(
            router,
            axum::http::Request::get(constants_str::VALUE_688DB289)
                .body(axum::body::Body::empty())
                .expect(constants_str::DIAGNOSTIC_69409C47),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_34ACCF01);
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert!(
            response
                .headers()
                .get(axum::http::header::CONTENT_TYPE)
                .and_then(|header| header.to_str().ok())
                .is_some_and(|header| header.contains(constants_str::TEXT_CSS))
        );
        assert_eq!(
            response
                .headers()
                .get(constants_str::X_CONTENT_TYPE_OPTIONS),
            Some(&axum::http::HeaderValue::from_static(
                constants_str::NOSNIFF
            ))
        );
        assert!(
            axum::body::to_bytes(response.into_body(), constants_usize::VALUE_1_048_576)
                .await
                .expect(constants_str::DIAGNOSTIC_1A1131EA)
                .len()
                > 1_000usize
        );
    }

    #[tokio::test]
    async fn test_api_body_limit_accepts_exact_boundary_and_preserves_operational_routes() {
        let check_limit = async |http_body_maximum_bytes: crate::http_body_maximum_bytes::HttpBodyMaximumBytes| {
            let limit = http_body_maximum_bytes.get();
            let operational_path = common_routes::common_route::CommonRoute::HealthLive.path();
            let body_handler = async |bytes| {
                (
                    axum::http::StatusCode::OK,
                    axum::body::Bytes::len(&bytes).to_string(),
                )
            };
            let router = axum::Router::from(crate::mount_service_routes::mount_service_routes(
                server_runtime_http::axum_router::AxumRouter::from(
                    axum::Router::new()
                        .route(operational_path.as_ref(), axum::routing::post(body_handler)),
                ),
                crate::axum_api_routes::AxumApiRoutes::from(axum::Router::new().route(
                    constants_str::VALUE_87D0B7F8,
                    axum::routing::post(body_handler),
                )),
                http_body_maximum_bytes,
            ));
            let send_body = async |path, length, expected_status| {
                let request_result = axum::http::Request::post(path)
                    .body(axum::body::Body::from(vec![0u8; length]));
                assert!(request_result.as_ref().is_ok_and(|request| request.method() == axum::http::Method::POST));
                let Ok(request) = request_result else {
                    return;
                };
                let response_result = tower::ServiceExt::oneshot(router.clone(), request).await;
                assert!(response_result.as_ref().is_ok_and(|response| response.status() == expected_status));
                let Ok(response) = response_result;
                if expected_status == axum::http::StatusCode::OK {
                    let body = axum::body::to_bytes(response.into_body(), 1_024usize).await;
                    assert!(body.is_ok_and(|bytes| bytes.as_ref() == length.to_string().as_bytes()));
                }
            };
            tokio::join!(
                send_body(constants_str::VALUE_87D0B7F8, limit.saturating_sub(1usize), axum::http::StatusCode::OK),
                send_body(constants_str::VALUE_87D0B7F8, limit, axum::http::StatusCode::OK),
                send_body(constants_str::VALUE_87D0B7F8, limit + 1usize, axum::http::StatusCode::PAYLOAD_TOO_LARGE),
                send_body(operational_path.as_ref(), limit + 1usize, axum::http::StatusCode::OK),
            );
        };
        tokio::join!(
            check_limit(crate::http_body_maximum_bytes::HttpBodyMaximumBytes::from(
                0usize
            )),
            check_limit(crate::http_body_maximum_bytes::HttpBodyMaximumBytes::from(
                1usize
            )),
            check_limit(crate::http_body_maximum_bytes::HttpBodyMaximumBytes::from(
                4usize
            )),
            check_limit(crate::http_body_maximum_bytes::HttpBodyMaximumBytes::from(
                1_024usize
            )),
        );
    }

    #[tokio::test]
    async fn test_operational_and_api_routes_are_root_mounted() {
        let operational_path = common_routes::common_route::CommonRoute::HealthLive.path();
        let router = axum::Router::from(crate::mount_service_routes::mount_service_routes(
            server_runtime_http::axum_router::AxumRouter::from(
                axum::Router::new()
                    .route(
                        operational_path.as_ref(),
                        axum::routing::get(async || axum::http::StatusCode::NO_CONTENT),
                    )
                    .fallback(async || axum::http::StatusCode::IM_A_TEAPOT),
            ),
            crate::axum_api_routes::AxumApiRoutes::from(axum::Router::new().route(
                constants_str::VALUE_87D0B7F8,
                axum::routing::get(async || constants_str::VALUE_14C2529E),
            )),
            crate::http_body_maximum_bytes::HttpBodyMaximumBytes::from(1_024usize),
        ))
        .merge(axum::Router::from(
            crate::frontend_fallback_routes::frontend_fallback_routes(),
        ));
        let status = |path: &str| {
            tower::ServiceExt::oneshot(
                router.clone(),
                axum::http::Request::builder()
                    .uri(path)
                    .body(axum::body::Body::empty())
                    .expect(constants_str::DIAGNOSTIC_7496F84F),
            )
        };
        assert_eq!(
            status(operational_path.as_ref())
                .await
                .expect(constants_str::DIAGNOSTIC_0A94FCC5)
                .status(),
            axum::http::StatusCode::NO_CONTENT
        );
        assert_eq!(
            status(constants_str::VALUE_87D0B7F8)
                .await
                .expect(constants_str::DIAGNOSTIC_6BB8E3F5)
                .status(),
            axum::http::StatusCode::OK
        );
        assert_eq!(
            status(constants_str::VALUE_BB6C49D8)
                .await
                .map(|response| response.status()),
            Ok(axum::http::StatusCode::SEE_OTHER)
        );
        assert_eq!(
            status(constants_str::VALUE_A04A495F)
                .await
                .expect(constants_str::DIAGNOSTIC_11FD3E4A)
                .status(),
            axum::http::StatusCode::SEE_OTHER
        );
        assert_eq!(
            status(constants_str::VALUE_3C3BEAFC)
                .await
                .expect(constants_str::DIAGNOSTIC_6E17DB87)
                .status(),
            axum::http::StatusCode::SEE_OTHER
        );
    }

    #[tokio::test]
    async fn test_missing_page_redirects_to_default_authentication_page() {
        let response = tower::ServiceExt::oneshot(
            axum::Router::from(crate::frontend_fallback_routes::frontend_fallback_routes()),
            axum::http::Request::builder()
                .uri(constants_str::VALUE_10D40EF4)
                .body(axum::body::Body::empty())
                .expect(constants_str::DIAGNOSTIC_CFE228D8),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_BD9F2B00);
        assert_eq!(response.status(), axum::http::StatusCode::SEE_OTHER);
        assert_eq!(
            response.headers().get(axum::http::header::LOCATION),
            Some(&axum::http::HeaderValue::from_static(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::SignIn.get()
            ))
        );
    }
    #[test]
    fn test_tracing_default_filter_is_stable() {
        assert_eq!(
            constants_str::CONFIG_TRACING_INFO,
            std::str::from_utf8(b"info").expect(constants_str::VALUE_0EF05B85)
        );
    }

    #[tokio::test]
    async fn test_administrator_asset_router_preserves_head_method_and_missing_path_responses() {
        let router =
            axum::Router::from(frontend_admin::admin_frontend_routes::admin_frontend_routes());
        let missing_path = format!(
            "{}/{}",
            server_admin_contract::admin_frontend_path::AdminFrontendPath::Assets.get(),
            stringify!(test_missing_administrator_asset)
        );
        let missing_uri_result = missing_path.parse::<axum::http::Uri>();
        assert!(missing_uri_result.is_ok());
        let Ok(missing_uri) = missing_uri_result else {
            return;
        };
        let request_response = async |method, uri| {
            let mut request = axum::http::Request::new(axum::body::Body::empty());
            *request.method_mut() = method;
            *request.uri_mut() = uri;
            tower::ServiceExt::oneshot(router.clone(), request).await
        };
        let (head_result, post_result, missing_result, outside_result) = tokio::join!(
            request_response(
                axum::http::Method::HEAD,
                axum::http::Uri::from_static(constants_str::VALUE_688DB289)
            ),
            request_response(
                axum::http::Method::POST,
                axum::http::Uri::from_static(constants_str::VALUE_688DB289)
            ),
            request_response(axum::http::Method::GET, missing_uri),
            request_response(
                axum::http::Method::GET,
                axum::http::Uri::from_static(
                    server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get()
                )
            ),
        );
        assert!(head_result.as_ref().is_ok_and(|response| {
            response.status() == axum::http::StatusCode::OK
                && response
                    .headers()
                    .get(axum::http::header::CONTENT_TYPE)
                    .and_then(|header| header.to_str().ok())
                    .is_some_and(|header| header.contains(constants_str::TEXT_CSS))
        }));
        assert_eq!(
            post_result.map(|response| response.status()),
            Ok(axum::http::StatusCode::METHOD_NOT_ALLOWED)
        );
        assert_eq!(
            missing_result.map(|response| response.status()),
            Ok(axum::http::StatusCode::NOT_FOUND)
        );
        assert_eq!(
            outside_result.map(|response| response.status()),
            Ok(axum::http::StatusCode::NOT_FOUND)
        );
        let Ok(head_response) = head_result;
        assert!(
            axum::body::to_bytes(head_response.into_body(), 1usize)
                .await
                .is_ok_and(|body| body.is_empty())
        );
    }

    #[tokio::test]
    async fn test_postgresql_pool_rejects_inverted_limits_before_client_construction() {
        fn server_pool_fixture_env_value<T>(
            std_env_var_ok_ref: config_lib::std_env_var_ok_ref::StdEnvVarOkRef<'_>,
        ) -> Result<T, crate::server_io_error::ServerIoError>
        where
            T: config_lib::try_from_std_env_var_ok::TryFromStdEnvVarOk,
            T::Error: std::error::Error + Send + Sync + 'static,
        {
            let std_env_var_ok = config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                std_env_var_ok_ref.as_ref().to_owned(),
            )
            .map_err(std::io::Error::other)
            .map_err(crate::server_io_error::ServerIoError::from)?;
            T::try_from_std_env_var_ok(std_env_var_ok)
                .map_err(std::io::Error::other)
                .map_err(crate::server_io_error::ServerIoError::from)
        }
        let outcome = async {
            let mut server_config = server_config::server_config::ServerConfig::new(
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::ASTERISK,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::TEST_CONTENT_SECURITY_POLICY,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::POSTGRES_DB,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::TEST_AUDIENCE,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::TEST_ISSUER,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::VALUE_127_0_0_1_32_PATH_1_128,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::VALUE_900,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::VALUE_4,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::VALUE_10,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::VALUE_2592000,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::VALUE_20,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::VALUE_10,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::TEST_VALUE_30,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::TEST_VALUE_30,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::TEST_VALUE_30,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::TEST_VALUE_30,
            ))?,
            config_lib::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytes::try_from(
                16_384usize,
            )
            .map_err(std::io::Error::other)
            .map_err(crate::server_io_error::ServerIoError::from)?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::VALUE_127_0_0_1_8080,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::VALUE_1,
            ))?,
            config_lib::pg_pool_min_connections::PgPoolMinConnections::from(2u32),
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::VALUE_0,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::GITHUB_ALT,
            ))?,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::CONFIG_TRACING_INFO,
            ))?,
            config_lib::tracing_format::TracingFormat::Text,
            server_pool_fixture_env_value(config_lib::std_env_var_ok_ref::StdEnvVarOkRef::from(
                constants_str::TRUE,
            ))?,
            config_lib::admin_cookie_secure::AdminCookieSecure::from(false),
            config_lib::admin_swagger_enabled::AdminSwaggerEnabled::from(true),
            config_lib::http_gzip_enabled::HttpGzipEnabled::from(true),
            config_lib::production_mode::ProductionMode::from(false),
            config_lib::service_mode::ServiceMode::Serve,
        );
            assert!(matches!(
                crate::make_postgresql_pool::make_postgresql_pool(&server_config).await,
                Err(crate::run_server_error::RunServerError::PgPoolConfiguration)
            ));
            *server_config.get_pg_pool_min_connections_mut() =
                config_lib::pg_pool_min_connections::PgPoolMinConnections::from(u32::MAX);
            *server_config.get_pg_pool_max_connections_mut() =
                config_lib::pg_pool_max_connections::PgPoolMaxConnections::try_from(
                    u32::MAX - 1u32,
                )
                .map_err(std::io::Error::other)
                .map_err(crate::server_io_error::ServerIoError::from)?;
            assert!(matches!(
                crate::make_postgresql_pool::make_postgresql_pool(&server_config).await,
                Err(crate::run_server_error::RunServerError::PgPoolConfiguration)
            ));
            Ok::<(), crate::server_io_error::ServerIoError>(())
        }
        .await;
        assert!(matches!(outcome, Ok(())));
    }

    #[tokio::test]
    async fn test_admin_metrics_handlers_preserve_empty_renderer_text_and_html_responses() {
        let metrics_exporter_prometheus_renderer =
            crate::metrics_exporter_prometheus_renderer::MetricsExporterPrometheusRenderer::from(
                metrics_exporter_prometheus::PrometheusBuilder::new()
                    .build_recorder()
                    .handle(),
            );
        let expected_text = metrics_exporter_prometheus::PrometheusHandle::from(
            metrics_exporter_prometheus_renderer.clone(),
        )
        .render();
        assert!(expected_text.is_empty());
        let (mut parts, ()) = axum::http::Request::new(()).into_parts();
        let extracted_result = <crate::axum_metrics_exporter_prometheus_renderer::AxumMetricsExporterPrometheusRenderer as axum::extract::FromRequestParts<crate::metrics_exporter_prometheus_renderer::MetricsExporterPrometheusRenderer>>::from_request_parts(
            &mut parts,
            &metrics_exporter_prometheus_renderer,
        ).await;
        assert!(extracted_result.is_ok());
        let Ok(extracted) = extracted_result;
        assert_eq!(parts.method, axum::http::Method::GET);
        assert_eq!(parts.uri.path(), constants_str::SLASH);
        assert!(parts.headers.is_empty());
        let metrics_result = crate::admin_metrics::admin_metrics(extracted).await;
        assert!(metrics_result.is_ok());
        let Ok(metrics_response) = metrics_result else {
            return;
        };
        assert_eq!(metrics_response.status(), axum::http::StatusCode::OK);
        assert!(
            metrics_response
                .headers()
                .get(axum::http::header::CONTENT_TYPE)
                .is_some_and(|header| header
                    == format!(
                        "{}/{}; {}={}-{}",
                        stringify!(text),
                        stringify!(plain),
                        stringify!(charset),
                        stringify!(utf),
                        8u8
                    )
                    .as_str())
        );
        assert!(
            axum::body::to_bytes(metrics_response.into_body(), 1usize)
                .await
                .is_ok_and(|body| body.as_ref() == expected_text.as_bytes())
        );
        let title_result = frontend_admin::admin_ssr_text::AdminSsrText::try_from(
            constants_str::METRICS_ALT.to_owned(),
        );
        let text_result = frontend_admin::admin_ssr_text::AdminSsrText::try_from(expected_text);
        assert!(title_result.is_ok());
        assert!(text_result.is_ok());
        let (Ok(title), Ok(text)) = (title_result, text_result) else {
            return;
        };
        let expected_html = String::from(frontend_admin::render_text_page::render_text_page(
            server_admin_contract::admin_page::AdminPage::Metrics,
            title,
            text,
        ));
        let html_response = crate::admin_metrics_page::admin_metrics_page(
            crate::axum_metrics_exporter_prometheus_renderer::AxumMetricsExporterPrometheusRenderer::from(
                metrics_exporter_prometheus_renderer,
            ),
        )
        .await;
        assert_eq!(html_response.status(), axum::http::StatusCode::OK);
        assert!(
            html_response
                .headers()
                .get(axum::http::header::CONTENT_TYPE)
                .is_some_and(|header| header
                    == format!(
                        "{}/{}; {}={}-{}",
                        stringify!(text),
                        stringify!(html),
                        stringify!(charset),
                        stringify!(utf),
                        8u8
                    )
                    .as_str())
        );
        assert!(
            axum::body::to_bytes(html_response.into_body(), constants_usize::VALUE_8_388_608)
                .await
                .is_ok_and(|body| !body.is_empty() && body.as_ref() == expected_html.as_bytes())
        );
    }

    #[test]
    fn test_admin_metrics_render_error_returns_empty_internal_server_error() {
        let response = axum::response::IntoResponse::into_response(
            crate::admin_metrics_error::AdminMetricsError::Render(
                server_runtime_http::metrics_response_body_error::MetricsResponseBodyError::TooLarge,
            ),
        );
        assert_eq!(
            response.status(),
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            axum::body::HttpBody::size_hint(response.body()).exact(),
            Some(0u64)
        );
        assert!(
            response
                .headers()
                .get(axum::http::header::CONTENT_TYPE)
                .is_none()
        );
    }

    #[tokio::test]
    async fn test_admin_openapi_json_response_preserves_document_and_content_type() {
        let document = crate::admin_open_api::admin_open_api().await;
        let expected = document.0.to_json();
        let response = axum::response::IntoResponse::into_response(document);
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(
            response.headers().get(axum::http::header::CONTENT_TYPE),
            Some(&axum::http::HeaderValue::from_static(
                constants_str::APPLICATION_JSON
            ))
        );
        let collected =
            axum::body::to_bytes(response.into_body(), constants_usize::VALUE_8_388_608).await;
        assert!(expected.is_ok_and(|expected_text| {
            collected.is_ok_and(|body| body.as_ref() == expected_text.as_bytes())
        }));
    }
}
