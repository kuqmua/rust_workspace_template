#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdministratorAccountCommandError {
    #[error(transparent)]
    Args(crate::AdministratorCommandArgsError),
    #[error("failed to read configuration: {0}")]
    Config(server_config::domain_types::ConfigTryFromEnvError),
    #[error("unsafe production configuration: {0}")]
    ConfigProduction(server_config::domain_types::ProductionConfigError),
    #[error("failed to connect to postgres: {0}")]
    Connect(crate::SqlxAdministratorDatabaseConnectionError),
    #[error("failed to create the first administrator: {0}")]
    InitialAdministratorCreation(server_admin::domain_types::InitialAdministratorCreationError),
    #[error("failed to prepare administrator schema: {0}")]
    Migrate(server_admin::domain_types::AdminMigrateError),
    #[error("failed to read initial administrator creation password file: {0}")]
    PasswordFile(server_runtime_http::domain_types::BoundedReadError),
    #[error("initial administrator creation password file is invalid")]
    PasswordFileValue,
    #[error("failed to reset the administrator password: {0}")]
    PasswordReset(server_admin::domain_types::AdminPasswordResetError),
}
