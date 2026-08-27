#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("read query plan exceeds the query fragment limit")]
pub struct ReadQueryPlanError;
