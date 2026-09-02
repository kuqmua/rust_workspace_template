#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum MultipartRequestError {
    #[error("multipart request payload exceeds its maximum")]
    PayloadTooLarge,
    #[error("multipart request contains too many parts")]
    TooManyParts,
}
