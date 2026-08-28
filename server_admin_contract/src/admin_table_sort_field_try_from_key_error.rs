#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq, thiserror::Error,
)]
#[error("{}", constants_str::UNKNOWN_ADMIN_TABLE_SORT_FIELD)]
pub struct AdminTableSortFieldTryFromKeyError;
