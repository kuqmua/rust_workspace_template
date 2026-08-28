#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct InitialAdministratorCreationArgs {
    pub display_name: server_admin::domain_types::AdminDisplayName,
    pub login: server_admin::domain_types::AdminLogin,
    pub password_file: crate::AdministratorPasswordFilePathBuf,
}

impl InitialAdministratorCreationArgs {
    pub(crate) fn into_parts(
        self,
    ) -> (
        server_admin::domain_types::AdminDisplayName,
        server_admin::domain_types::AdminLogin,
        crate::AdministratorPasswordFilePathBuf,
    ) {
        (self.display_name, self.login, self.password_file)
    }
}
