pub(crate) async fn make_postgresql_pool(
    config: &server_config::server_config::ServerConfig,
) -> Result<app_state::sqlx_pg_pool::SqlxPgPool, crate::run_server_error::RunServerError> {
    if *config.pg_pool_min_connections
        > *config_lib::pg_pool_max_connections::PgPoolMaxConnectionsProvider::pg_pool_max_connections(config)
    {
        return Err(crate::run_server_error::RunServerError::PgPoolConfiguration);
    }
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(
            *config_lib::pg_pool_max_connections::PgPoolMaxConnectionsProvider::pg_pool_max_connections(
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
        .map(app_state::sqlx_pg_pool::SqlxPgPool::from)
        .map_err(|error| {
            crate::run_server_error::RunServerError::PgConnect(
                crate::sqlx_server_pg_connect_error::SqlxServerPgConnectError::from(error),
            )
        })
}
