#[derive(proc_macro_new::New, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct InitialAdministratorCreationArgs {
    display_name: server_admin_contract::admin_display_name::AdminDisplayName,
    login: server_admin_contract::admin_login::AdminLogin,
    password_file: crate::administrator_password_file_path_buf::AdministratorPasswordFilePathBuf,
}

impl InitialAdministratorCreationArgs {
    pub(crate) fn into_parts(
        self,
    ) -> (
        server_admin_contract::admin_display_name::AdminDisplayName,
        server_admin_contract::admin_login::AdminLogin,
        crate::administrator_password_file_path_buf::AdministratorPasswordFilePathBuf,
    ) {
        (self.display_name, self.login, self.password_file)
    }
}
