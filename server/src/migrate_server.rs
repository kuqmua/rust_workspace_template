#[allow(clippy::single_call_fn)] // named command or composition stage has one orchestration owner
pub(crate) async fn migrate_server(
    config: &server_config::domain_types::Config,
) -> Result<(), crate::domain_types::RunServerError> {
    let pg_pool = crate::make_postgresql_pool::make_postgresql_pool(config).await?;
    server_admin::domain_types::prepare_postgresql(app_state::domain_types::SqlxPgPoolRef::from(
        pg_pool.as_ref(),
    ))
    .await
    .map_err(|error| {
        crate::domain_types::RunServerError::PrepAdminPg(
            crate::domain_types::ServerAdminMigrateError::from(error),
        )
    })
}
