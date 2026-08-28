pub(crate) fn error_status(
    error: &crate::AdministratorAccountCommandError,
) -> crate::AdministratorAccountCommandStatus {
    crate::AdministratorAccountCommandStatus::from(match error {
        crate::AdministratorAccountCommandError::Args(_)
        | crate::AdministratorAccountCommandError::PasswordFileValue => 2u8,
        crate::AdministratorAccountCommandError::InitialAdministratorCreation(
            server_admin::domain_types::InitialAdministratorCreationError::AlreadyInitialized,
        ) => 3u8,
        crate::AdministratorAccountCommandError::Config(_)
        | crate::AdministratorAccountCommandError::ConfigProduction(_)
        | crate::AdministratorAccountCommandError::Connect(_)
        | crate::AdministratorAccountCommandError::Migrate(_)
        | crate::AdministratorAccountCommandError::PasswordFile(_)
        | crate::AdministratorAccountCommandError::PasswordReset(_)
        | crate::AdministratorAccountCommandError::InitialAdministratorCreation(_) => 1u8,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn exit_codes_distinguish_invalid_input_and_completed_initial_administrator_creation() {
        assert_eq!(
            super::error_status(&crate::AdministratorAccountCommandError::Args(
                crate::AdministratorCommandArgsError::Usage,
            )),
            crate::AdministratorAccountCommandStatus::from(2u8)
        );
        assert_eq!(
            super::error_status(&crate::AdministratorAccountCommandError::InitialAdministratorCreation(
                server_admin::domain_types::InitialAdministratorCreationError::AlreadyInitialized,
            )),
            crate::AdministratorAccountCommandStatus::from(3u8)
        );
    }
}
