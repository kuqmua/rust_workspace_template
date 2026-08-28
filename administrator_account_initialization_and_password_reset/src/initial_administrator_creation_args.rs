#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct InitialAdministratorCreationArgs {
    display_name: server_admin::domain_types::AdminDisplayName,
    login: server_admin::domain_types::AdminLogin,
    password_file: crate::domain_types::AdministratorPasswordFilePathBuf,
}

impl InitialAdministratorCreationArgs {
    pub(crate) fn into_parts(
        self,
    ) -> (
        server_admin::domain_types::AdminDisplayName,
        server_admin::domain_types::AdminLogin,
        crate::domain_types::AdministratorPasswordFilePathBuf,
    ) {
        (self.display_name, self.login, self.password_file)
    }

    pub(crate) const fn new(
        display_name: server_admin::domain_types::AdminDisplayName,
        login: server_admin::domain_types::AdminLogin,
        password_file: crate::domain_types::AdministratorPasswordFilePathBuf,
    ) -> Self {
        Self {
            display_name,
            login,
            password_file,
        }
    }
}
