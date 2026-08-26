mod adapters;
mod domain_types;
mod run_server;

fn main() -> domain_types::ServerExitCode {
    let config = match server_config::domain_types::Config::try_from_env() {
        Ok(config) => config,
        Err(config_error) => {
            let startup_error = domain_types::RunServerError::Config(
                domain_types::ServerConfigError::from(config_error),
            );
            tracing::error!(error = %startup_error, "server configuration failed");
            return domain_types::ServerExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    if let Err(error) = config.validate_for_startup() {
        tracing::error!(
            error = %domain_types::RunServerError::ConfigProduction(domain_types::ServerConfigProductionError::from(error)),
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
                error = %domain_types::RunServerError::ObservabilityInit(domain_types::ServerObservabilityInitError::from(error)),
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
                tokio::runtime::Runtime::from(runtime)
                    .block_on(run_server::migrate_server::migrate_server(&config))
            }
            config_lib::domain_types::types::SvcMode::Serve => {
                tokio::runtime::Runtime::from(runtime).block_on(run_server::run_server(config))
            }
        });
    if let Err(error) = run_result.as_ref() {
        tracing::error!(error = %error, "server terminated with an error");
    }
    let shutdown_result = observability.shutdown().map_err(|error| {
        domain_types::RunServerError::ObservabilityShutdown(
            domain_types::ServerObservabilityShutdownError::from(error),
        )
    });
    match run_result.and(shutdown_result) {
        Ok(()) => domain_types::ServerExitCode::from(std::process::ExitCode::SUCCESS),
        Err(error) => {
            tracing::error!(error = %error, "server operation or observability shutdown failed");
            domain_types::ServerExitCode::from(std::process::ExitCode::FAILURE)
        }
    }
}
