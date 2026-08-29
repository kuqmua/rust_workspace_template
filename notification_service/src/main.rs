// The owner module retains lint-sensitive semantics from the original implementation.

#![allow(clippy::arbitrary_source_item_ordering)] // OpenAPI document stays next to its generated schema and operation marker
#![allow(clippy::wildcard_imports)] // root-owned binary modules share the former domain owner facade vocabulary

mod axum_notification_json;
mod axum_notification_response;
mod axum_notification_router;
mod axum_notification_state;
mod create_notification;
mod create_notification_error;
pub(crate) use axum_notification_json::*;
pub(crate) use axum_notification_response::*;
pub(crate) use axum_notification_router::*;
pub(crate) use axum_notification_state::*;
pub(crate) use create_notification_error::*;
pub(crate) use http_notification_status_code::*;
pub(crate) use metrics_error::*;
pub(crate) use metrics_exporter_prometheus_notification_build_error::*;
pub(crate) use metrics_exporter_prometheus_renderer::*;
pub(crate) use notification_body_maximum_bytes::*;
pub(crate) use notification_error_code::*;
pub(crate) use notification_exit_code::*;
pub(crate) use notification_io_error::*;
pub(crate) use notification_service_error::*;
pub(crate) use notification_state::*;
pub(crate) use sqlx_notification_database_error::*;
pub(crate) use sqlx_notification_migration_error::*;
mod http_notification_status_code;
mod metrics;
mod metrics_error;
mod metrics_exporter_prometheus_notification_build_error;
mod metrics_exporter_prometheus_renderer;
mod notification_api_route_registry;
mod notification_body_maximum_bytes;
mod notification_error_code;
mod notification_exit_code;
mod notification_io_error;
mod notification_route_registry;
mod notification_service_error;
mod notification_state;
mod open_api;
#[cfg(test)]
mod open_api_document;
mod router;
#[cfg(test)]
pub(crate) use open_api_document::open_api_document;
pub(crate) use router::build_notification_router;
mod sqlx_notification_database_error;
mod sqlx_notification_migration_error;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> NotificationExitCode {
    let config = match notification_service_config::config::Config::try_from_env() {
        Ok(value) => value,
        Err(error) => {
            tracing::error!(
                error = %NotificationServiceError::Config(error),
                "notification service configuration failed"
            );
            return NotificationExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    let tracing_format =
        if *config.tracing_format() == config_lib::domain_types::types::TracingFormat::Json {
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
                error = %NotificationServiceError::ObservabilityInit(error),
                "notification service observability initialization failed"
            );
            return NotificationExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    let run_result = match config.svc_mode() {
        config_lib::domain_types::types::SvcMode::Migrate => {
            (async {
            let pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(**config.pg_pool_max_connections())
                .connect(secrecy::ExposeSecret::expose_secret(
                    &config.notification_database_url().0,
                ))
                .await
                .map_err(|error| {
                    NotificationServiceError::Database(
                        SqlxNotificationDatabaseError::from(error),
                    )
                })?;
            sqlx::migrate!("../notification_service_migrations")
                .run(&pool)
                .await
                .map_err(|error| {
                    NotificationServiceError::Migration(
                        SqlxNotificationMigrationError::from(error),
                    )
                })
            })
            .await
        }
        config_lib::domain_types::types::SvcMode::Serve => {
            (async {
            let metrics = metrics_exporter_prometheus::PrometheusBuilder::new()
                .install_recorder()
                .map(MetricsExporterPrometheusRenderer::from)
                .map_err(|error| {
                    NotificationServiceError::Metrics(
                        MetricsExporterPrometheusNotificationBuildError::from(error),
                    )
                })?;
            let pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(**config.pg_pool_max_connections())
                .connect(secrecy::ExposeSecret::expose_secret(
                    &config.notification_database_url().0,
                ))
                .await
                .map_err(|error| {
                    NotificationServiceError::Database(
                        SqlxNotificationDatabaseError::from(error),
                    )
                })?;
            let listener =
                tokio::net::TcpListener::bind(config.notification_service_socket_address().0)
                    .await
                    .map_err(|error| {
                        NotificationServiceError::Socket(NotificationIoError::from(error))
                    })?;
            let actual_service_socket_address = listener.local_addr().map_err(|error| {
                NotificationServiceError::Socket(NotificationIoError::from(error))
            })?;
            let timeout = server_runtime_http::domain_types::RequestTimeoutDuration::try_from(
                std::time::Duration::from_secs(config.request_timeout_seconds().get()),
            )
            .map_err(|_error| NotificationServiceError::Timeout)?;
            let service_router = server_runtime_http::domain_types::RequestIdLayer::with_span_config(
                server_runtime_http::domain_types::HttpRequestSpanConfig::new(
                    server_runtime_http::domain_types::ServiceName::from(env!("CARGO_PKG_NAME")),
                    server_runtime_http::domain_types::ClientSocketAddr::from(actual_service_socket_address),
                    server_runtime_http::domain_types::TrustedProxyRanges::default(),
                ),
            )
            .apply(
                server_runtime_http::domain_types::SecurityHeadersLayer::from(
                    server_runtime_http::domain_types::ForwardedProtoTrust::Ignore,
                )
                .apply(
                    server_runtime_http::domain_types::RequestTimeoutLayer::from(timeout).apply(
                        server_runtime_http::domain_types::AxumRouter::from(
                            build_notification_router(
                                NotificationState::new(
                                    metrics,
                                    app_state::SqlxPgPool::from(pool),
                                    git_info::project_git_info_value(),
                                ),
                                NotificationBodyMaximumBytes::from(
                                    notification_service_contract::domain_types::NOTIFICATION_API_BODY_MAX_BYTES,
                                ),
                            )
                            .into_inner(),
                        ),
                    ),
                ),
            );
            server_runtime_http::domain_types::serve_with_graceful_shutdown(
                server_runtime_http::domain_types::TokioTcpListener::from(listener),
                service_router,
                async {
                    if let Err(error) =
                        server_runtime_http::domain_types::wait_for_service_shutdown_signal().await
                    {
                        tracing::error!(error = %error, "notification shutdown signal failed");
                    }
                },
                timeout,
            )
            .await
            .map_err(NotificationServiceError::Serve)
            })
            .await
        }
    };
    if let Err(error) = run_result.as_ref() {
        tracing::error!(error = %error, "notification service terminated with an error");
    }
    let shutdown_result = observability
        .shutdown()
        .map_err(NotificationServiceError::ObservabilityShutdown);
    match run_result.and(shutdown_result) {
        Ok(()) => NotificationExitCode::from(std::process::ExitCode::SUCCESS),
        Err(error) => {
            tracing::error!(
                error = %error,
                "notification service operation or observability shutdown failed"
            );
            NotificationExitCode::from(std::process::ExitCode::FAILURE)
        }
    }
}
