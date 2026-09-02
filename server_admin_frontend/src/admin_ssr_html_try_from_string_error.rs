#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum AdminSsrHtmlTryFromStringError {
    #[error("administrator SSR HTML exceeds the size limit")]
    TooLarge,
}
