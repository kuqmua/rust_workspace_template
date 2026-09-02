#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum HttpProxyPathError {
    #[error("proxy path must not be empty")]
    Empty,
    #[error("proxy path contains forbidden segment")]
    ForbiddenSegment,
    #[error("proxy path contains forbidden syntax")]
    ForbiddenSyntax,
}
