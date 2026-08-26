mod adapters;
mod domain_types;

#[tokio::main]
async fn main() -> domain_types::NotificationExitCode {
    let config = match notification_service_config::config::Config::try_from_env() {
        Ok(value) => value,
        Err(error) => {
            tracing::error!(
                error = %domain_types::NotificationServiceError::Config(
                    domain_types::NotificationConfigError::from(error)
                ),
                "notification service configuration failed"
            );
            return domain_types::NotificationExitCode::from(std::process::ExitCode::FAILURE);
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
                error = %domain_types::NotificationServiceError::ObservabilityInit(
                    domain_types::NotificationObservabilityInitError::from(error)
                ),
                "notification service observability initialization failed"
            );
            return domain_types::NotificationExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    let run_result = match config.svc_mode() {
        config_lib::domain_types::types::SvcMode::Migrate => {
            adapters::runtime::migrate_notification(&config).await
        }
        config_lib::domain_types::types::SvcMode::Serve => adapters::runtime::run(config).await,
    };
    if let Err(error) = run_result.as_ref() {
        tracing::error!(error = %error, "notification service terminated with an error");
    }
    let shutdown_result = observability.shutdown().map_err(|error| {
        domain_types::NotificationServiceError::ObservabilityShutdown(
            domain_types::NotificationObservabilityShutdownError::from(error),
        )
    });
    match run_result.and(shutdown_result) {
        Ok(()) => domain_types::NotificationExitCode::from(std::process::ExitCode::SUCCESS),
        Err(error) => {
            tracing::error!(
                error = %error,
                "notification service operation or observability shutdown failed"
            );
            domain_types::NotificationExitCode::from(std::process::ExitCode::FAILURE)
        }
    }
}
