#![allow(clippy::single_call_fn)] // separate same-named owner module preserves command-status mapping ownership
#[allow(
    clippy::missing_const_for_fn,
    reason = "repository wrappers initialize through the non-const From trait"
)]
pub(crate) fn error_status(
    error: &crate::domain_types::AdministratorAccountCommandError,
) -> crate::domain_types::AdministratorAccountCommandStatus {
    crate::domain_types::AdministratorAccountCommandStatus::from(match error {
        crate::domain_types::AdministratorAccountCommandError::Args(_)
        | crate::domain_types::AdministratorAccountCommandError::PasswordFileValue => 2u8,
        crate::domain_types::AdministratorAccountCommandError::InitialAdministratorCreation(
            server_admin::domain_types::InitialAdministratorCreationError::AlreadyInitialized,
        ) => 3u8,
        crate::domain_types::AdministratorAccountCommandError::Config(_)
        | crate::domain_types::AdministratorAccountCommandError::ConfigProduction(_)
        | crate::domain_types::AdministratorAccountCommandError::Connect(_)
        | crate::domain_types::AdministratorAccountCommandError::Migrate(_)
        | crate::domain_types::AdministratorAccountCommandError::PasswordFile(_)
        | crate::domain_types::AdministratorAccountCommandError::PasswordReset(_)
        | crate::domain_types::AdministratorAccountCommandError::InitialAdministratorCreation(_) => {
            1u8
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn exit_codes_distinguish_invalid_input_and_completed_initial_administrator_creation() {
        assert_eq!(
            super::error_status(
                &crate::domain_types::AdministratorAccountCommandError::Args(
                    crate::domain_types::AdministratorCommandArgsError::Usage,
                )
            ),
            crate::domain_types::AdministratorAccountCommandStatus::from(2u8)
        );
        assert_eq!(
            super::error_status(&crate::domain_types::AdministratorAccountCommandError::InitialAdministratorCreation(
                server_admin::domain_types::InitialAdministratorCreationError::AlreadyInitialized,
            )),
            crate::domain_types::AdministratorAccountCommandStatus::from(3u8)
        );
    }
}
