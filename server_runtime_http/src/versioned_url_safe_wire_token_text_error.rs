#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum VersionedUrlSafeWireTokenTextError {
    #[error("wire token contains an invalid URL-safe part")]
    InvalidPart,
    #[error("wire token has an invalid structure")]
    InvalidStructure,
    #[error("wire token is too long")]
    TooLong,
}
