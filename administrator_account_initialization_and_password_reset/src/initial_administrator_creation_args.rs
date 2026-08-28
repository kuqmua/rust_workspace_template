#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct InitialAdministratorCreationArgs {
    display_name: server_admin::domain_types::AdminDisplayName,
    login: server_admin::domain_types::AdminLogin,
    password_file: super::AdministratorPasswordFilePathBuf,
}

impl InitialAdministratorCreationArgs {
    pub(crate) fn into_parts(
        self,
    ) -> (
        server_admin::domain_types::AdminDisplayName,
        server_admin::domain_types::AdminLogin,
        super::AdministratorPasswordFilePathBuf,
    ) {
        (self.display_name, self.login, self.password_file)
    }

    #[allow(clippy::single_call_fn)] // the application parser has one construction site for this command model
    pub(crate) const fn new(
        display_name: server_admin::domain_types::AdminDisplayName,
        login: server_admin::domain_types::AdminLogin,
        password_file: super::AdministratorPasswordFilePathBuf,
    ) -> Self {
        Self {
            display_name,
            login,
            password_file,
        }
    }
}
