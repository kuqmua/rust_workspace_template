#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum AdminSecretTextError {
    #[error("administrator secret text has invalid bounds")]
    InvalidBounds,
    #[error("administrator secret text is too short")]
    TooShort,
    #[error("administrator secret text is too long")]
    TooLong,
    #[error("administrator secret text contains a NUL character")]
    ContainsNul,
    #[error("administrator secret text has an invalid value")]
    InvalidValue,
}
