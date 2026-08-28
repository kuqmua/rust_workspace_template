#[allow(clippy::single_call_fn)] // named command or composition stage has one orchestration owner
pub(crate) async fn run_admin_account_command() -> Result<
    server_admin::domain_types::AdminUserId,
    crate::domain_types::AdministratorAccountCommandError,
> {
    let command = crate::parse_args::parse_args()
        .map_err(crate::domain_types::AdministratorAccountCommandError::Args)?;
    let config = server_config::domain_types::Config::try_from_env()
        .map_err(crate::domain_types::AdministratorAccountCommandError::Config)?;
    config
        .validate_for_startup()
        .map_err(crate::domain_types::AdministratorAccountCommandError::ConfigProduction)?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1u32)
        .connect(secrecy::ExposeSecret::expose_secret(
            config_lib::domain_types::DatabaseUrlProvider::database_url(&config),
        ))
        .await
        .map_err(|error| {
            crate::domain_types::AdministratorAccountCommandError::Connect(
                crate::domain_types::SqlxAdministratorDatabaseConnectionError::from(error),
            )
        })?;
    server_admin::domain_types::prepare_postgresql(app_state::domain_types::SqlxPgPoolRef::from(
        &pool,
    ))
    .await
    .map_err(crate::domain_types::AdministratorAccountCommandError::Migrate)?;
    let concurrency = std::num::NonZeroUsize::new(config.admin_password_hash_concurrency.get())
        .ok_or(crate::domain_types::AdministratorAccountCommandError::PasswordFileValue)?;
    let password_hasher = server_admin::domain_types::AdminPasswordHasher::new(
        server_admin::domain_types::AdminPasswordHashConcurrency::from(
            server_admin::domain_types::AdminNonZeroUsize::from(concurrency),
        ),
    );
    match command {
        crate::domain_types::AdminCommand::CreateInitialAdministrator(args) => {
            let (display_name, login, password_file) = args.into_parts();
            let password = crate::password_from_file::password_from_file(&password_file)?;
            server_admin::domain_types::create_initial_administrator(
                app_state::domain_types::SqlxPgPoolRef::from(&pool),
                login,
                display_name,
                password,
                &password_hasher,
            )
            .await
            .map_err(
                crate::domain_types::AdministratorAccountCommandError::InitialAdministratorCreation,
            )
        }
        crate::domain_types::AdminCommand::PasswordReset(args) => {
            let (login, password_file) = args.into_parts();
            let password = crate::password_from_file::password_from_file(&password_file)?;
            server_admin::domain_types::reset_admin_password(
                app_state::domain_types::SqlxPgPoolRef::from(&pool),
                login,
                password,
                &password_hasher,
            )
            .await
            .map_err(crate::domain_types::AdministratorAccountCommandError::PasswordReset)
        }
    }
}
