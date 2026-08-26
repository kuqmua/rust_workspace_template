#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AdministratorPasswordFilePathBuf(std::path::PathBuf);

impl AdministratorPasswordFilePathBuf {
    pub(crate) fn as_path_ref(&self) -> server_runtime_http::domain_types::PathRef<'_> {
        server_runtime_http::domain_types::PathRef::from(self.0.as_path())
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct SqlxAdministratorDatabaseConnectionError(sqlx::Error);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct AdministratorAccountCommandStatus(u8);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct InitialAdministratorCreationArgs {
    display_name: server_admin::domain_types::AdminDisplayName,
    login: server_admin::domain_types::AdminLogin,
    password_file: AdministratorPasswordFilePathBuf,
}

impl InitialAdministratorCreationArgs {
    pub(crate) fn into_parts(
        self,
    ) -> (
        server_admin::domain_types::AdminDisplayName,
        server_admin::domain_types::AdminLogin,
        AdministratorPasswordFilePathBuf,
    ) {
        (self.display_name, self.login, self.password_file)
    }

    #[allow(clippy::single_call_fn)] // the application parser has one construction site for this command model
    pub(crate) const fn new(
        display_name: server_admin::domain_types::AdminDisplayName,
        login: server_admin::domain_types::AdminLogin,
        password_file: AdministratorPasswordFilePathBuf,
    ) -> Self {
        Self {
            display_name,
            login,
            password_file,
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct PasswordResetArgs {
    login: server_admin::domain_types::AdminLogin,
    password_file: AdministratorPasswordFilePathBuf,
}

impl PasswordResetArgs {
    pub(crate) fn into_parts(
        self,
    ) -> (
        server_admin::domain_types::AdminLogin,
        AdministratorPasswordFilePathBuf,
    ) {
        (self.login, self.password_file)
    }

    #[allow(clippy::single_call_fn)] // the application parser has one construction site for this command model
    pub(crate) const fn new(
        login: server_admin::domain_types::AdminLogin,
        password_file: AdministratorPasswordFilePathBuf,
    ) -> Self {
        Self {
            login,
            password_file,
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) enum AdminCommand {
    CreateInitialAdministrator(InitialAdministratorCreationArgs),
    PasswordReset(PasswordResetArgs),
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdministratorCommandArgsError {
    #[error("initial administrator creation display name is invalid")]
    DisplayName,
    #[error("initial administrator creation login is invalid")]
    Login,
    #[error(
        "usage: administrator_account_initialization_and_password_reset <login> <display_name> <password_file> | administrator_account_initialization_and_password_reset reset <login> <password_file>; password_file must contain only the new password"
    )]
    Usage,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdministratorAccountCommandError {
    #[error(transparent)]
    Args(AdministratorCommandArgsError),
    #[error("failed to read configuration: {0}")]
    Config(server_config::domain_types::ConfigTryFromEnvError),
    #[error("unsafe production configuration: {0}")]
    ConfigProduction(server_config::domain_types::ProductionConfigError),
    #[error("failed to connect to postgres: {0}")]
    Connect(SqlxAdministratorDatabaseConnectionError),
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

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(crate) struct AdministratorAccountCommandExitCode(std::process::ExitCode);

impl std::process::Termination for AdministratorAccountCommandExitCode {
    fn report(self) -> std::process::ExitCode {
        self.0
    }
}
