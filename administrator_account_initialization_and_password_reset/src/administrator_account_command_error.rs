#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdministratorAccountCommandError {
    #[error(transparent)]
    Args(crate::administrator_command_args_error::AdministratorCommandArgsError),
    #[error("failed to read configuration: {0}")]
    Config(server_config::server_config::ServerConfigTryFromEnvError),
    #[error("unsafe production configuration: {0}")]
    ConfigProduction(server_config::production_config_error::ProductionConfigError),
    #[error("failed to connect to postgres: {0}")]
    Connect(crate::sqlx_administrator_database_connection_error::SqlxAdministratorDatabaseConnectionError),
    #[error("failed to create the first administrator: {0}")]
    InitialAdministratorCreation(
        server_admin::initial_administrator_creation_error::InitialAdministratorCreationError,
    ),
    #[error("failed to prepare administrator schema: {0}")]
    Migrate(server_admin::admin_migrate_error::AdminMigrateError),
    #[error("failed to read initial administrator creation password file: {0}")]
    PasswordFile(server_runtime_http::bounded_read_error::BoundedReadError),
    #[error("initial administrator creation password file is invalid")]
    PasswordFileValue,
    #[error("failed to reset the administrator password: {0}")]
    PasswordReset(server_admin::admin_password_reset_error::AdminPasswordResetError),
}
