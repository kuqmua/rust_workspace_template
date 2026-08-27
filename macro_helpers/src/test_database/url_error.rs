#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum UrlError {
    #[error("database name is not explicitly test-only: {target}")]
    AmbiguousDatabase {
        target: super::SanitizedDatabaseTarget,
    },
    #[error("test database URL is malformed")]
    Malformed,
    #[error("test database host is not loopback: {target}")]
    NonLoopback {
        target: super::SanitizedDatabaseTarget,
    },
}
