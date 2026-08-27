#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdministratorCommandArgsError {
    #[error("initial administrator creation display name is invalid")]
    DisplayName,
    #[error("initial administrator creation login is invalid")]
    Login,
    #[error(
        "usage: administrator_account_initialization_and_password_reset <login> <display_name> <password_file> | administrator_account_initialization_and_password_reset reset <login> <password_file>; password_file must contain only the new password"
    )]
    Usage,
}
