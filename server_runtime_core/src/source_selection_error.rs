#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum SourceSelectionError {
    #[error("{}", constants_str::SOURCE_SELECTION_REQUIRES_AT_LEAST_ONE_SOURCE)]
    Missing,
}
