#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum UrlSafeTokenPartTextError {
    #[error("URL-safe token part must not be empty")]
    Empty,
    #[error("URL-safe token part contains a forbidden symbol")]
    InvalidSymbol,
    #[error("URL-safe token part is too long")]
    TooLong,
}
