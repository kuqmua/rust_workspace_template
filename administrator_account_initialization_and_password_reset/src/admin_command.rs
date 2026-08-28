#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) enum AdminCommand {
    CreateInitialAdministrator(super::InitialAdministratorCreationArgs),
    PasswordReset(super::PasswordResetArgs),
}
