#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) enum AdminCommand {
    CreateInitialAdministrator(
        crate::initial_administrator_creation_args::InitialAdministratorCreationArgs,
    ),
    PasswordReset(crate::password_reset_args::PasswordResetArgs),
}
