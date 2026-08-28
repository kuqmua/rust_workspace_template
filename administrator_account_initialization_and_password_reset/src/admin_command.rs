#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) enum AdminCommand {
    CreateInitialAdministrator(crate::InitialAdministratorCreationArgs),
    PasswordReset(crate::PasswordResetArgs),
}
