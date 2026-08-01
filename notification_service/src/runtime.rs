async fn shutdown_signal() {
    if let Err(error) = server_runtime_http::wait_for_service_shutdown_signal().await {
        tracing::error!(error = %error, "notification shutdown signal failed");
    }
}

pub(super) async fn run(
    config: notification_service_config::Config,
) -> Result<(), super::NotificationServiceError> {
    let metrics = metrics_exporter_prometheus::PrometheusBuilder::new()
        .install_recorder()
        .map(super::MetricsExporterPrometheusHandle)
        .map_err(|error| {
            super::NotificationServiceError::Metrics(
                super::MetricsExporterPrometheusNotificationBuildError::from(error),
            )
        })?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(**config.pg_pool_max_connections())
        .connect(secrecy::ExposeSecret::expose_secret(
            &config.notification_database_url().0,
        ))
        .await
        .map_err(|error| {
            super::NotificationServiceError::Database(super::SqlxNotificationDatabaseError::from(
                error,
            ))
        })?;
    let listener = tokio::net::TcpListener::bind(config.notification_service_socket_address().0)
        .await
        .map_err(|error| {
            super::NotificationServiceError::Socket(super::StdNotificationIoError::from(error))
        })?;
    let actual_service_socket_address = listener.local_addr().map_err(|error| {
        super::NotificationServiceError::Socket(super::StdNotificationIoError::from(error))
    })?;
    let timeout = server_runtime_http::StdRequestTimeout::try_from(std::time::Duration::from_secs(
        config.request_timeout_seconds().get(),
    ))
    .map_err(|_error| super::NotificationServiceError::Timeout)?;
    let service_router = server_runtime_http::RequestIdLayer::with_span_config(
        server_runtime_http::HttpRequestSpanConfig::new(
            server_runtime_http::ServiceName::from(env!("CARGO_PKG_NAME")),
            server_runtime_http::StdSocketAddr::from(actual_service_socket_address),
            server_runtime_http::TrustedProxyRanges::default(),
        ),
    )
    .apply(
        server_runtime_http::SecurityHeadersLayer::from(
            server_runtime_http::ForwardedProtoTrust::Ignore,
        )
        .apply(
            server_runtime_http::RequestTimeoutLayer::from(timeout).apply(
                server_runtime_http::AxumRouter::from(
                    super::routes::router(
                        super::NotificationState {
                            metrics,
                            pool: app_state::SqlxPgPool::from(pool),
                            project_git_info: git_info::project_git_info(),
                        },
                        super::NotificationBodyMaximumBytes::from(
                            notification_service_contract::NOTIFICATION_API_BODY_MAX_BYTES,
                        ),
                    )
                    .0,
                ),
            ),
        ),
    );
    server_runtime_http::serve_with_graceful_shutdown(
        server_runtime_http::TokioTcpListener::from(listener),
        service_router,
        shutdown_signal(),
        timeout,
    )
    .await
    .map_err(|error| {
        super::NotificationServiceError::Serve(super::NotificationServeError::from(error))
    })
}

pub(super) async fn migrate_notification(
    config: &notification_service_config::Config,
) -> Result<(), super::NotificationServiceError> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(**config.pg_pool_max_connections())
        .connect(secrecy::ExposeSecret::expose_secret(
            &config.notification_database_url().0,
        ))
        .await
        .map_err(|error| {
            super::NotificationServiceError::Database(super::SqlxNotificationDatabaseError::from(
                error,
            ))
        })?;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|error| {
            super::NotificationServiceError::Migration(super::SqlxNotificationMigrationError::from(
                error,
            ))
        })
}
