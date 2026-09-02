pub(crate) fn error_status(
    administrator_account_command_error: &crate::administrator_account_command_error::AdministratorAccountCommandError,
) -> crate::administrator_account_command_status::AdministratorAccountCommandStatus {
    crate::administrator_account_command_status::AdministratorAccountCommandStatus::from(match administrator_account_command_error {
        crate::administrator_account_command_error::AdministratorAccountCommandError::Args(_)
        | crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFileValue => 2u8,
        crate::administrator_account_command_error::AdministratorAccountCommandError::InitialAdministratorCreation(
            server_admin::initial_administrator_creation_error::InitialAdministratorCreationError::AlreadyInitialized,
        ) => 3u8,
        crate::administrator_account_command_error::AdministratorAccountCommandError::Config(_)
        | crate::administrator_account_command_error::AdministratorAccountCommandError::ConfigProduction(_)
        | crate::administrator_account_command_error::AdministratorAccountCommandError::Connect(_)
        | crate::administrator_account_command_error::AdministratorAccountCommandError::Migrate(_)
        | crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFile(_)
        | crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordReset(_)
        | crate::administrator_account_command_error::AdministratorAccountCommandError::InitialAdministratorCreation(_) => 1u8,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_exit_codes_distinguish_invalid_input_and_completed_initial_administrator_creation() {
        assert_eq!(
            crate::error_status::error_status(
                &crate::administrator_account_command_error::AdministratorAccountCommandError::Args(
                    crate::administrator_command_args_error::AdministratorCommandArgsError::Usage,
                )
            ),
            crate::administrator_account_command_status::AdministratorAccountCommandStatus::from(
                2u8
            )
        );
        assert_eq!(
            crate::error_status::error_status(&crate::administrator_account_command_error::AdministratorAccountCommandError::InitialAdministratorCreation(
                server_admin::initial_administrator_creation_error::InitialAdministratorCreationError::AlreadyInitialized,
            )),
            crate::administrator_account_command_status::AdministratorAccountCommandStatus::from(3u8)
        );
    }
}
