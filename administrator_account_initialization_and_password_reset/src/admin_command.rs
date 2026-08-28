#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) enum AdminCommand {
    CreateInitialAdministrator(crate::domain_types::InitialAdministratorCreationArgs),
    PasswordReset(crate::domain_types::PasswordResetArgs),
}
