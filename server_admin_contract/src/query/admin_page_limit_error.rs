use super::AdminPageLimit;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq, thiserror::Error,
)]
#[error(
    "administrator page limit must be between {min} and {max}",
    min = AdminPageLimit::MIN,
    max = AdminPageLimit::MAX
)]
pub struct AdminPageLimitError;
