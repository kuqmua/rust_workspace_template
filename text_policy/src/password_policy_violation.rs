#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum PasswordPolicyViolation {
    #[error("password must not contain whitespace")]
    ContainsWhitespace,
    #[error("password must contain a digit")]
    MissingDigit,
    #[error("password must contain a lowercase letter")]
    MissingLowercase,
    #[error("password must contain a special character")]
    MissingSpecial,
    #[error("password must contain an uppercase letter")]
    MissingUppercase,
    #[error("password is too long")]
    TooLong,
    #[error("password is too short")]
    TooShort,
}
