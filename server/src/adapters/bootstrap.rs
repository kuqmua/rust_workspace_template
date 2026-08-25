#![allow(clippy::single_call_fn)] // bootstrap helpers each own one validated construction responsibility

pub(crate) fn mk_app_state(
    config: server_config::domain_types::Config,
    pg_pool: app_state::domain_types::SqlxPgPool,
) -> crate::domain_types::SharedServerAppStateArc {
    crate::domain_types::SharedServerAppStateArc::from(std::sync::Arc::new(
        server_app_state::domain_types::ServerAppState {
            bulk_item_budget: server_runtime_http::domain_types::ResourceBudget::new(
                server_runtime_http::domain_types::ResourceBudgetMaximum::from(
                    std::num::NonZeroUsize::new(4_096usize).unwrap_or(std::num::NonZeroUsize::MIN),
                ),
            ),
            config,
            idempotency_response_budget: server_runtime_http::domain_types::ResourceBudget::new(
                server_runtime_http::domain_types::ResourceBudgetMaximum::from(
                    std::num::NonZeroUsize::new(
                        64usize.saturating_mul(constants_usize::VALUE_1_048_576),
                    )
                    .unwrap_or(std::num::NonZeroUsize::MIN),
                ),
            ),
            pg_pool,
            project_git_info: git_info::domain_types::project_git_info(),
        },
    ))
}

pub(crate) fn mk_runtime()
-> Result<crate::domain_types::TokioServerRuntime, crate::domain_types::RunServerError> {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
        .map(crate::domain_types::TokioServerRuntime::from)
        .map_err(|error| {
            crate::domain_types::RunServerError::BuildRuntime(
                crate::domain_types::ServerIoError::from(error),
            )
        })
}

pub(crate) async fn mk_pg_pool(
    config: &server_config::domain_types::Config,
) -> Result<app_state::domain_types::SqlxPgPool, crate::domain_types::RunServerError> {
    if *config.pg_pool_min_connections
        > *config_lib::domain_types::GetPgPoolMaxConnections::get_pg_pool_max_connections(config)
    {
        return Err(crate::domain_types::RunServerError::PgPoolConfiguration);
    }
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(
            *config_lib::domain_types::GetPgPoolMaxConnections::get_pg_pool_max_connections(config),
        )
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
                    constants_str::POSTGRES_STATEMENT_TIMEOUT_SQL,
                )
                .await
                .map(drop)
            })
        })
        .connect(secrecy::ExposeSecret::expose_secret(
            config_lib::domain_types::GetDatabaseUrl::get_database_url(config),
        ))
        .await
        .map(app_state::domain_types::SqlxPgPool::from)
        .map_err(|error| {
            crate::domain_types::RunServerError::PgConnect(
                crate::domain_types::SqlxServerPgConnectError::from(error),
            )
        })
}
