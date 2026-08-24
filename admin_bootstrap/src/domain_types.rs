#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct BootstrapPathBuf(std::path::PathBuf);

impl BootstrapPathBuf {
    pub(crate) fn as_path_ref(&self) -> server_runtime_http::PathRef<'_> {
        server_runtime_http::PathRef::from(self.0.as_path())
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct SqlxBootstrapError(sqlx::Error);

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
pub(crate) struct BootstrapStatus(u8);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct BootstrapArgs {
    display_name: server_admin::AdminDisplayName,
    login: server_admin::AdminLogin,
    password_file: BootstrapPathBuf,
}

impl BootstrapArgs {
    pub(crate) fn into_parts(
        self,
    ) -> (
        server_admin::AdminDisplayName,
        server_admin::AdminLogin,
        BootstrapPathBuf,
    ) {
        (self.display_name, self.login, self.password_file)
    }

    pub(crate) const fn new(
        display_name: server_admin::AdminDisplayName,
        login: server_admin::AdminLogin,
        password_file: BootstrapPathBuf,
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
    login: server_admin::AdminLogin,
    password_file: BootstrapPathBuf,
}

impl PasswordResetArgs {
    pub(crate) fn into_parts(self) -> (server_admin::AdminLogin, BootstrapPathBuf) {
        (self.login, self.password_file)
    }

    pub(crate) const fn new(
        login: server_admin::AdminLogin,
        password_file: BootstrapPathBuf,
    ) -> Self {
        Self {
            login,
            password_file,
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) enum AdminCommand {
    Bootstrap(BootstrapArgs),
    PasswordReset(PasswordResetArgs),
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum BootstrapArgsError {
    #[error("administrator bootstrap display name is invalid")]
    DisplayName,
    #[error("administrator bootstrap login is invalid")]
    Login,
    #[error(
        "usage: admin_bootstrap <login> <display_name> <password_file> | admin_bootstrap reset <login> <password_file>; password_file must contain only the new password"
    )]
    Usage,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum BootstrapCommandError {
    #[error(transparent)]
    Args(BootstrapArgsError),
    #[error("failed to create the first administrator: {0}")]
    Bootstrap(server_admin::AdminBootstrapError),
    #[error("failed to read configuration: {0}")]
    Config(server_config::domain_types::ConfigTryFromEnvError),
    #[error("unsafe production configuration: {0}")]
    ConfigProduction(server_config::domain_types::ProductionConfigError),
    #[error("failed to connect to postgres: {0}")]
    Connect(SqlxBootstrapError),
    #[error("failed to prepare administrator schema: {0}")]
    Migrate(server_admin::AdminMigrateError),
    #[error("failed to read administrator bootstrap password file: {0}")]
    PasswordFile(server_runtime_http::BoundedReadError),
    #[error("administrator bootstrap password file is invalid")]
    PasswordFileValue,
    #[error("failed to reset the administrator password: {0}")]
    PasswordReset(server_admin::AdminPasswordResetError),
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(crate) struct BootstrapExitCode(std::process::ExitCode);

impl std::process::Termination for BootstrapExitCode {
    fn report(self) -> std::process::ExitCode {
        self.0
    }
}
