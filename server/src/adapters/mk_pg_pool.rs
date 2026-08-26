#![allow(clippy::single_call_fn)] // database pool construction helpers each own one validated construction responsibility

pub(crate) async fn mk_pg_pool(
    config: &server_config::domain_types::Config,
) -> Result<app_state::domain_types::SqlxPgPool, crate::domain_types::RunServerError> {
    if *config.pg_pool_min_connections
        > *config_lib::domain_types::PgPoolMaxConnectionsProvider::pg_pool_max_connections(config)
    {
        return Err(crate::domain_types::RunServerError::PgPoolConfiguration);
    }
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(
            *config_lib::domain_types::PgPoolMaxConnectionsProvider::pg_pool_max_connections(
                config,
            ),
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
            config_lib::domain_types::DatabaseUrlProvider::database_url(config),
        ))
        .await
        .map(app_state::domain_types::SqlxPgPool::from)
        .map_err(|error| {
            crate::domain_types::RunServerError::PgConnect(
                crate::domain_types::SqlxServerPgConnectError::from(error),
            )
        })
}
