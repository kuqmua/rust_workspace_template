#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("environment initializer string value is invalid")]
pub(crate) struct InitStringError;
