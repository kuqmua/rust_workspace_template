#![allow(clippy::single_call_fn)] // bootstrap helpers each own one validated construction responsibility

pub(super) fn mk_app_state(
    config: server_config::Config,
    pg_pool: app_state::SqlxPgPool,
) -> super::StdSharedServerAppState {
    super::StdSharedServerAppState::from(std::sync::Arc::new(server_app_state::ServerAppState {
        bulk_item_budget: server_runtime_http::ResourceBudget::new(
            server_runtime_http::ResourceBudgetMaximum::from(
                std::num::NonZeroUsize::new(4_096usize).unwrap_or(std::num::NonZeroUsize::MIN),
            ),
        ),
        config,
        idempotency_response_budget: server_runtime_http::ResourceBudget::new(
            server_runtime_http::ResourceBudgetMaximum::from(
                std::num::NonZeroUsize::new(
                    64usize.saturating_mul(usize_constants::VALUE_1_048_576),
                )
                .unwrap_or(std::num::NonZeroUsize::MIN),
            ),
        ),
        pg_pool,
        project_git_info: git_info::project_git_info(),
    }))
}

pub(super) fn mk_runtime() -> Result<super::TokioServerRuntime, super::RunServerError> {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
        .map(super::TokioServerRuntime)
        .map_err(|error| super::RunServerError::BuildRuntime(super::StdServerIoError::from(error)))
}

pub(super) async fn mk_pg_pool(
    config: &server_config::Config,
) -> Result<app_state::SqlxPgPool, super::RunServerError> {
    if *config.pg_pool_min_connections
        > *config_lib::GetPgPoolMaxConnections::get_pg_pool_max_connections(config)
    {
        return Err(super::RunServerError::PgPoolConfiguration);
    }
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(*config_lib::GetPgPoolMaxConnections::get_pg_pool_max_connections(config))
        .min_connections(*config.pg_pool_min_connections)
        .acquire_timeout(std::time::Duration::from_secs(
            config.pg_pool_acquire_timeout_seconds.get(),
        ))
        .idle_timeout(std::time::Duration::from_secs(
            config.pg_pool_idle_timeout_seconds.get(),
        ))
        .max_lifetime(std::time::Duration::from_secs(
            config.pg_pool_max_lifetime_seconds.get(),
        ))
        .after_connect(|connection, _metadata| {
            Box::pin(async move {
                sqlx::Executor::execute(
                    &mut *connection,
                    str_constants::POSTGRES_STATEMENT_TIMEOUT_SQL,
                )
                .await
                .map(drop)
            })
        })
        .connect(secrecy::ExposeSecret::expose_secret(
            config_lib::GetDatabaseUrl::get_database_url(config),
        ))
        .await
        .map(app_state::SqlxPgPool::from)
        .map_err(|error| {
            super::RunServerError::PgConnect(super::SqlxServerPgConnectError::from(error))
        })
}
