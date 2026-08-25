#![allow(clippy::single_call_fn)] // the binary entrypoint has one application composition owner

#[tokio::main]
pub(crate) async fn run_main() -> crate::domain_types::NotificationExitCode {
    let config = match notification_service_config::domain_types::Config::try_from_env() {
        Ok(value) => value,
        Err(error) => {
            tracing::error!(
                error = %crate::domain_types::NotificationServiceError::Config(
                    crate::domain_types::NotificationConfigError::from(error)
                ),
                "notification service configuration failed"
            );
            return crate::domain_types::NotificationExitCode::from(
                std::process::ExitCode::FAILURE,
            );
        }
    };
    let tracing_format =
        if *config.tracing_format() == config_lib::domain_types::types::TracingFormat::Json {
            server_runtime_http::domain_types::ServiceTracingFormat::Json
        } else {
            server_runtime_http::domain_types::ServiceTracingFormat::Text
        };
    let observability = match server_runtime_http::domain_types::initialize_service_observability(
        tracing_format,
        server_runtime_http::domain_types::ServiceName::from(env!("CARGO_PKG_NAME")),
    ) {
        Ok(value) => value,
        Err(error) => {
            tracing::error!(
                error = %crate::domain_types::NotificationServiceError::ObservabilityInit(
                    crate::domain_types::NotificationObservabilityInitError::from(error)
                ),
                "notification service observability initialization failed"
            );
            return crate::domain_types::NotificationExitCode::from(
                std::process::ExitCode::FAILURE,
            );
        }
    };
    let run_result = match config.svc_mode() {
        config_lib::domain_types::types::SvcMode::Migrate => {
            crate::adapters::runtime::migrate_notification(&config).await
        }
        config_lib::domain_types::types::SvcMode::Serve => {
            crate::adapters::runtime::run(config).await
        }
    };
    if let Err(error) = run_result.as_ref() {
        tracing::error!(error = %error, "notification service terminated with an error");
    }
    let shutdown_result = observability.shutdown().map_err(|error| {
        crate::domain_types::NotificationServiceError::ObservabilityShutdown(
            crate::domain_types::NotificationObservabilityShutdownError::from(error),
        )
    });
    match run_result.and(shutdown_result) {
        Ok(()) => crate::domain_types::NotificationExitCode::from(std::process::ExitCode::SUCCESS),
        Err(error) => {
            tracing::error!(
                error = %error,
                "notification service operation or observability shutdown failed"
            );
            crate::domain_types::NotificationExitCode::from(std::process::ExitCode::FAILURE)
        }
    }
}
