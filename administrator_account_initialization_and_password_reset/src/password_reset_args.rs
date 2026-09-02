#[derive(proc_macro_new::New, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct PasswordResetArgs {
    login: server_admin_contract::admin_login::AdminLogin,
    password_file: crate::administrator_password_file_path_buf::AdministratorPasswordFilePathBuf,
}

impl PasswordResetArgs {
    pub(crate) fn into_parts(
        self,
    ) -> (
        server_admin_contract::admin_login::AdminLogin,
        crate::administrator_password_file_path_buf::AdministratorPasswordFilePathBuf,
    ) {
        (self.login, self.password_file)
    }
}
