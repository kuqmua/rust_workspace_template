#[allow(
    clippy::single_call_fn,
    reason = "migration mode remains isolated from the long-running service startup path"
)]
async fn migrate_server(
    config: &server_config::domain_types::Config,
) -> Result<(), crate::domain_types::RunServerError> {
    let pg_pool = crate::adapters::bootstrap::mk_pg_pool(config).await?;
    server_admin::domain_types::prep_pg(app_state::domain_types::SqlxPgPoolRef::from(
        pg_pool.as_ref(),
    ))
    .await
    .map_err(|error| {
        crate::domain_types::RunServerError::PrepAdminPg(
            crate::domain_types::ServerAdminMigrateError::from(error),
        )
    })
}

#[allow(
    clippy::single_call_fn,
    reason = "the executable adapter delegates server startup to its owned module"
)]
pub(crate) fn run_main() -> crate::domain_types::ServerExitCode {
    let config = match server_config::domain_types::Config::try_from_env() {
        Ok(config) => config,
        Err(config_error) => {
            let startup_error = crate::domain_types::RunServerError::Config(
                crate::domain_types::ServerConfigError::from(config_error),
            );
            tracing::error!(error = %startup_error, "server configuration failed");
            return crate::domain_types::ServerExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    if let Err(error) = config.validate_for_startup() {
        tracing::error!(
            error = %crate::domain_types::RunServerError::ConfigProduction(crate::domain_types::ServerConfigProductionError::from(error)),
            "server production configuration validation failed"
        );
        return crate::domain_types::ServerExitCode::from(std::process::ExitCode::FAILURE);
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
                error = %crate::domain_types::RunServerError::ObservabilityInit(crate::domain_types::ServerObservabilityInitError::from(error)),
                "server observability initialization failed"
            );
            return crate::domain_types::ServerExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    let run_result = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
        .map(crate::domain_types::TokioServerRuntime::from)
        .map_err(|error| {
            crate::domain_types::RunServerError::BuildRuntime(
                crate::domain_types::ServerIoError::from(error),
            )
        })
        .and_then(|runtime| match config.svc_mode {
            config_lib::domain_types::types::SvcMode::Migrate => {
                tokio::runtime::Runtime::from(runtime).block_on(migrate_server(&config))
            }
            config_lib::domain_types::types::SvcMode::Serve => {
                tokio::runtime::Runtime::from(runtime).block_on(super::run_server(config))
            }
        });
    if let Err(error) = run_result.as_ref() {
        tracing::error!(error = %error, "server terminated with an error");
    }
    let shutdown_result = observability.shutdown().map_err(|error| {
        crate::domain_types::RunServerError::ObservabilityShutdown(
            crate::domain_types::ServerObservabilityShutdownError::from(error),
        )
    });
    match run_result.and(shutdown_result) {
        Ok(()) => crate::domain_types::ServerExitCode::from(std::process::ExitCode::SUCCESS),
        Err(error) => {
            tracing::error!(error = %error, "server operation or observability shutdown failed");
            crate::domain_types::ServerExitCode::from(std::process::ExitCode::FAILURE)
        }
    }
}
