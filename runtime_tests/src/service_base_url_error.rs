#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum ServiceBaseUrlError {
    #[error("service base URL must include a host")]
    Host,
    #[error("service base URL exceeds its maximum length")]
    Length,
    #[error("service base URL must use HTTP or HTTPS")]
    Scheme,
    #[error("service base URL must not include a query or fragment")]
    Suffix,
}
