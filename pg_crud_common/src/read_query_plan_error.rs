#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum ReadQueryPlanError {
    #[error("read query plan exceeds the query fragment limit")]
    TooManyFragments,
}
