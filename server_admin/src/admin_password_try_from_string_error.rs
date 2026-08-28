#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum AdminPasswordTryFromStringError {
    #[error("{}", constants_str::ADMINISTRATOR_PASSWORD_LENGTH_IS_INVALID)]
    InvalidLength,
}
