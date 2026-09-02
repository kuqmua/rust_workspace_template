#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub(crate) enum AdminJoinedTextTryFromStringError {
    #[error("joined administrator frontend text exceeds the size limit")]
    TooLong,
}
