#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct PasswordResetArgs {
    login: server_admin::domain_types::AdminLogin,
    password_file: super::AdministratorPasswordFilePathBuf,
}

impl PasswordResetArgs {
    pub(crate) fn into_parts(
        self,
    ) -> (
        server_admin::domain_types::AdminLogin,
        super::AdministratorPasswordFilePathBuf,
    ) {
        (self.login, self.password_file)
    }

    #[allow(clippy::single_call_fn)] // the application parser has one construction site for this command model
    pub(crate) const fn new(
        login: server_admin::domain_types::AdminLogin,
        password_file: super::AdministratorPasswordFilePathBuf,
    ) -> Self {
        Self {
            login,
            password_file,
        }
    }
}
