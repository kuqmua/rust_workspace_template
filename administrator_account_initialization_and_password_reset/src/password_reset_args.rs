#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct PasswordResetArgs {
    pub login: server_admin::domain_types::AdminLogin,
    pub password_file: crate::AdministratorPasswordFilePathBuf,
}

impl PasswordResetArgs {
    pub(crate) fn into_parts(
        self,
    ) -> (
        server_admin::domain_types::AdminLogin,
        crate::AdministratorPasswordFilePathBuf,
    ) {
        (self.login, self.password_file)
    }
}
