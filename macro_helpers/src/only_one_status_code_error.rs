#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    thiserror::Error,
)]
pub enum OnlyOneStatusCodeError {
    #[error("07286cf0: two or more supported status code attrs")]
    MoreThanOne,
    #[error("19fc6512: supported status code attr not found")]
    NotFound,
}
