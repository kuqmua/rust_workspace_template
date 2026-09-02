#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    thiserror::Error,
)]
pub enum AdminTableSortFieldTryFromKeyError {
    #[error("{}", constants_str::UNKNOWN_ADMIN_TABLE_SORT_FIELD)]
    Unknown,
}
