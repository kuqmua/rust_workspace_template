// The owner module retains lint-sensitive semantics from the original implementation.

#![allow(clippy::arbitrary_source_item_ordering)] // OpenAPI document stays next to its generated schema and operation marker
#![allow(clippy::wildcard_imports)] // root-owned binary modules share the former domain owner facade vocabulary

pub mod axum_notification_response;
pub mod build_notification_router;
pub mod create_notification;
pub mod create_notification_error;
pub mod domain_types;
pub mod http_notification_status_code;
pub mod metrics;
pub mod metrics_error;
pub mod metrics_exporter_prometheus_notification_build_error;
pub mod notification_api_route_registry;
pub mod notification_axum_json;
pub mod notification_axum_router;
pub mod notification_axum_state;
pub mod notification_body_maximum_bytes;
pub mod notification_error_code;
pub mod notification_exit_code;
pub mod notification_io_error;
pub mod notification_metrics_exporter_prometheus_renderer;
pub mod notification_open_api;
pub mod notification_route_registry;
pub mod notification_service_error;
pub mod notification_state;
#[cfg(test)]
pub mod open_api_document;
pub mod shared_notification_state_arc;
pub mod sqlx_notification_database_error;
pub mod sqlx_notification_migration_error;

#[cfg(test)]
pub mod tests;

#[tokio::main]
async fn main() -> notification_exit_code::NotificationExitCode {
    let config = match notification_service_config::notification_service_config::NotificationServiceConfig::try_from_env() {
        Ok(value) => value,
        Err(error) => {
            tracing::error!(
                error = %notification_service_error::NotificationServiceError::Config(error),
                message = %constants_str::TRACING_NOTIFICATION_CONFIG_FAILED,
            );
            return notification_exit_code::NotificationExitCode::from(
                std::process::ExitCode::FAILURE,
            );
        }
    };
    let tracing_format =
        if *config.tracing_format() == config_lib::tracing_format::TracingFormat::Json {
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
                    error = %notification_service_error::NotificationServiceError::ObservabilityInit(error),
                    message = %constants_str::TRACING_NOTIFICATION_OBSERVABILITY_INIT_FAILED,
                );
                return notification_exit_code::NotificationExitCode::from(
                    std::process::ExitCode::FAILURE,
                );
            }
        };
    let run_result = match config.svc_mode() {
        config_lib::svc_mode::SvcMode::Migrate => {
            (async {
            let pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(**config.pg_pool_max_connections())
                .connect(secrecy::ExposeSecret::expose_secret(
                    config.notification_database_url().get_inner(),
                ))
                .await
                .map_err(|error| {
                    notification_service_error::NotificationServiceError::Database(
                        sqlx_notification_database_error::SqlxNotificationDatabaseError::from(error),
                    )
                })?;
            sqlx::migrate!("../notification_service_migrations")
                .run(&pool)
                .await
                .map_err(|error| {
                    notification_service_error::NotificationServiceError::Migration(
                        sqlx_notification_migration_error::SqlxNotificationMigrationError::from(error),
                    )
                })
            })
            .await
        }
        config_lib::svc_mode::SvcMode::Serve => {
            (async {
            let metrics = metrics_exporter_prometheus::PrometheusBuilder::new()
                .install_recorder()
                .map(notification_metrics_exporter_prometheus_renderer::NotificationMetricsExporterPrometheusRenderer::from)
                .map_err(|error| {
                    notification_service_error::NotificationServiceError::Metrics(
                        metrics_exporter_prometheus_notification_build_error::MetricsExporterPrometheusNotificationBuildError::from(error),
                    )
                })?;
            let pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(**config.pg_pool_max_connections())
                .connect(secrecy::ExposeSecret::expose_secret(
                    config.notification_database_url().get_inner(),
                ))
                .await
                .map_err(|error| {
                    notification_service_error::NotificationServiceError::Database(
                        sqlx_notification_database_error::SqlxNotificationDatabaseError::from(error),
                    )
                })?;
            let listener =
                tokio::net::TcpListener::bind(*config.notification_service_socket_address().get_inner())
                    .await
                    .map_err(|error| {
                        notification_service_error::NotificationServiceError::Socket(notification_io_error::NotificationIoError::from(error))
                    })?;
            let actual_service_socket_address = listener.local_addr().map_err(|error| {
                notification_service_error::NotificationServiceError::Socket(notification_io_error::NotificationIoError::from(error))
            })?;
            let timeout = server_runtime_http::request_timeout_duration::RequestTimeoutDuration::try_from(
                std::time::Duration::from_secs(config.request_timeout_seconds().get()),
            )
            .map_err(|_error| notification_service_error::NotificationServiceError::Timeout)?;
            let service_router = server_runtime_http::request_id_layer::RequestIdLayer::with_span_config(
                server_runtime_http::http_request_span_config::HttpRequestSpanConfig::new(
                    server_observability::service_name::ServiceName::from(env!("CARGO_PKG_NAME")),
                    server_runtime_http::client_socket_addr::ClientSocketAddr::from(actual_service_socket_address),
                    server_runtime_http::trusted_proxy_ranges::TrustedProxyRanges::default(),
                ),
            )
            .apply(
                server_runtime_http::security_headers_layer::SecurityHeadersLayer::from(
                    server_runtime_http::forwarded_proto_trust::ForwardedProtoTrust::Ignore,
                )
                .apply(
                    server_runtime_http::request_timeout_layer::RequestTimeoutLayer::from(timeout).apply(
                        server_runtime_http::axum_router::AxumRouter::from(
                            build_notification_router::build_notification_router(
                                notification_state::NotificationState::new(
                                    metrics,
                                    app_state::sqlx_pg_pool::SqlxPgPool::from(pool),
                                    git_info::project_git_info_value::project_git_info_value(),
                                ),
                                notification_body_maximum_bytes::NotificationBodyMaximumBytes::from(
                                    notification_service_contract::notification_api_body_max_bytes::NOTIFICATION_API_BODY_MAX_BYTES,
                                ),
                            )
                            .into_inner(),
                        ),
                    ),
                ),
            );
            server_runtime_http::serve_with_graceful_shutdown::serve_with_graceful_shutdown(
                server_runtime_http::tokio_tcp_listener::TokioTcpListener::from(listener),
                service_router,
                async {
                    if let Err(error) =
                        server_runtime_http::wait_for_service_shutdown_signal::wait_for_service_shutdown_signal().await
                    {
                        tracing::error!(
                            error = %error,
                            message = %constants_str::TRACING_NOTIFICATION_SHUTDOWN_SIGNAL_FAILED,
                        );
                    }
                },
                timeout,
            )
            .await
            .map_err(notification_service_error::NotificationServiceError::Serve)
            })
            .await
        }
    };
    if let Err(error) = run_result.as_ref() {
        tracing::error!(
            error = %error,
            message = %constants_str::TRACING_NOTIFICATION_SERVICE_TERMINATED,
        );
    }
    let shutdown_result = observability
        .shutdown()
        .map_err(notification_service_error::NotificationServiceError::ObservabilityShutdown);
    match run_result.and(shutdown_result) {
        Ok(()) => {
            notification_exit_code::NotificationExitCode::from(std::process::ExitCode::SUCCESS)
        }
        Err(error) => {
            tracing::error!(
                error = %error,
                message = %constants_str::TRACING_NOTIFICATION_OPERATION_OR_SHUTDOWN_FAILED,
            );
            notification_exit_code::NotificationExitCode::from(std::process::ExitCode::FAILURE)
        }
    }
}
