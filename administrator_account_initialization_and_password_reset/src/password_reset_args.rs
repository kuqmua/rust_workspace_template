#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct PasswordResetArgs {
    login: server_admin::domain_types::AdminLogin,
    password_file: crate::domain_types::AdministratorPasswordFilePathBuf,
}

impl PasswordResetArgs {
    pub(crate) fn into_parts(
        self,
    ) -> (
        server_admin::domain_types::AdminLogin,
        crate::domain_types::AdministratorPasswordFilePathBuf,
    ) {
        (self.login, self.password_file)
    }

    pub(crate) const fn new(
        login: server_admin::domain_types::AdminLogin,
        password_file: crate::domain_types::AdministratorPasswordFilePathBuf,
    ) -> Self {
        Self {
            login,
            password_file,
        }
    }
}
