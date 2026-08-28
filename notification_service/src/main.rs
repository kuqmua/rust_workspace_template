// The owner module retains lint-sensitive semantics from the original implementation.

#![allow(clippy::single_call_fn)] // binary composition functions intentionally have one startup or route registration owner
#![allow(clippy::arbitrary_source_item_ordering)] // OpenAPI document stays next to its generated schema and operation marker
#![allow(clippy::needless_for_each)] // utoipa OpenApi derive expands to an internal for_each
#![allow(clippy::field_scoped_visibility_modifiers)] // sibling application and adapter modules consume these private binary domain models
#![allow(clippy::wildcard_imports)] // root-owned binary modules share the former domain owner facade vocabulary

mod axum_notification_json;
mod axum_notification_response;
mod axum_notification_router;
mod axum_notification_state;
mod create_notification;
mod create_notification_error;
mod domain_types;
pub(crate) use domain_types::*;
mod http_notification_status_code;
mod metrics;
mod metrics_error;
mod metrics_exporter_prometheus_notification_build_error;
mod metrics_exporter_prometheus_renderer;
mod migrate_notification;
mod notification_api_route_registry;
mod notification_body_maximum_bytes;
mod notification_config_error;
mod notification_error_code;
mod notification_exit_code;
mod notification_io_error;
mod notification_observability_init_error;
mod notification_observability_shutdown_error;
mod notification_route_registry;
mod notification_serve_error;
mod notification_service_error;
mod notification_state;
mod open_api;
#[cfg(test)]
mod open_api_document;
mod router;
mod routes;
mod run;
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
                error = %NotificationServiceError::Config(
                    NotificationConfigError::from(error)
                ),
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
                error = %NotificationServiceError::ObservabilityInit(
                    NotificationObservabilityInitError::from(error)
                ),
                "notification service observability initialization failed"
            );
            return NotificationExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    let run_result = match config.svc_mode() {
        config_lib::domain_types::types::SvcMode::Migrate => {
            migrate_notification::migrate_notification(&config).await
        }
        config_lib::domain_types::types::SvcMode::Serve => {
            run::run_notification_service(config).await
        }
    };
    if let Err(error) = run_result.as_ref() {
        tracing::error!(error = %error, "notification service terminated with an error");
    }
    let shutdown_result = observability.shutdown().map_err(|error| {
        NotificationServiceError::ObservabilityShutdown(
            NotificationObservabilityShutdownError::from(error),
        )
    });
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
