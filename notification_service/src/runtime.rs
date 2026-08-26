#![allow(clippy::single_call_fn)] // runtime adapter operations each have one application composition owner

async fn shutdown_signal() {
    if let Err(error) = server_runtime_http::domain_types::wait_for_service_shutdown_signal().await
    {
        tracing::error!(error = %error, "notification shutdown signal failed");
    }
}

pub(crate) async fn run(
    config: notification_service_config::config::Config,
) -> Result<(), crate::domain_types::NotificationServiceError> {
    let metrics = metrics_exporter_prometheus::PrometheusBuilder::new()
        .install_recorder()
        .map(crate::domain_types::MetricsExporterPrometheusRenderer::from)
        .map_err(|error| {
            crate::domain_types::NotificationServiceError::Metrics(
                crate::domain_types::MetricsExporterPrometheusNotificationBuildError::from(error),
            )
        })?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(**config.pg_pool_max_connections())
        .connect(secrecy::ExposeSecret::expose_secret(
            &config.notification_database_url().0,
        ))
        .await
        .map_err(|error| {
            crate::domain_types::NotificationServiceError::Database(
                crate::domain_types::SqlxNotificationDatabaseError::from(error),
            )
        })?;
    let listener = tokio::net::TcpListener::bind(config.notification_service_socket_address().0)
        .await
        .map_err(|error| {
            crate::domain_types::NotificationServiceError::Socket(
                crate::domain_types::NotificationIoError::from(error),
            )
        })?;
    let actual_service_socket_address = listener.local_addr().map_err(|error| {
        crate::domain_types::NotificationServiceError::Socket(
            crate::domain_types::NotificationIoError::from(error),
        )
    })?;
    let timeout = server_runtime_http::domain_types::RequestTimeoutDuration::try_from(
        std::time::Duration::from_secs(config.request_timeout_seconds().get()),
    )
    .map_err(|_error| crate::domain_types::NotificationServiceError::Timeout)?;
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
                    crate::adapters::routes::router(
                        crate::domain_types::NotificationState {
                            metrics,
                            pool: app_state::domain_types::SqlxPgPool::from(pool),
                            project_git_info: git_info::domain_types::project_git_info(),
                        },
                        crate::domain_types::NotificationBodyMaximumBytes::from(
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
        shutdown_signal(),
        timeout,
    )
    .await
    .map_err(|error| {
        crate::domain_types::NotificationServiceError::Serve(
            crate::domain_types::NotificationServeError::from(error),
        )
    })
}

pub(crate) async fn migrate_notification(
    config: &notification_service_config::config::Config,
) -> Result<(), crate::domain_types::NotificationServiceError> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(**config.pg_pool_max_connections())
        .connect(secrecy::ExposeSecret::expose_secret(
            &config.notification_database_url().0,
        ))
        .await
        .map_err(|error| {
            crate::domain_types::NotificationServiceError::Database(
                crate::domain_types::SqlxNotificationDatabaseError::from(error),
            )
        })?;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|error| {
            crate::domain_types::NotificationServiceError::Migration(
                crate::domain_types::SqlxNotificationMigrationError::from(error),
            )
        })
}
