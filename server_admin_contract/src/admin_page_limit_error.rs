#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq, thiserror::Error,
)]
#[error(
    "administrator page limit must be between {min} and {max}",
    min = crate::admin_page_limit::AdminPageLimit::MIN,
    max = crate::admin_page_limit::AdminPageLimit::MAX
)]
pub struct AdminPageLimitError;
