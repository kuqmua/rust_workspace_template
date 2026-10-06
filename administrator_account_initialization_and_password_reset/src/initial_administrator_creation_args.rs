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

#[cfg(test)]
mod tests {
    #[test]
    fn test_initial_administrator_arguments_preserve_each_field_on_ownership_transfer() {
        assert!(server_admin_contract::admin_display_name::AdminDisplayName::try_from(
            constants_str::ADMIN.to_owned(),
        ).is_ok_and(|admin_display_name| {
            server_admin_contract::admin_login::AdminLogin::try_from(constants_str::ROOT.to_owned())
                .is_ok_and(|admin_login| {
                    let administrator_password_file_path_buf = crate::administrator_password_file_path_buf::AdministratorPasswordFilePathBuf::from(std::path::PathBuf::from(constants_str::X));
                    let expected_password_file = format!("{administrator_password_file_path_buf:?}");
                    let arguments = crate::initial_administrator_creation_args::InitialAdministratorCreationArgs::new(admin_display_name, admin_login, administrator_password_file_path_buf);
                    let (transferred_display_name, transferred_login, transferred_password_file) = arguments.into_parts();
                    transferred_display_name.as_ref() == constants_str::ADMIN
                        && transferred_login.as_ref() == constants_str::ROOT
                        && format!("{transferred_password_file:?}") == expected_password_file
                })
        }));
    }
}
