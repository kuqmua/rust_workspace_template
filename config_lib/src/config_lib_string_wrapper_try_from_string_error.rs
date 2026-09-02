#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    thiserror::Error,
)]
pub enum ConfigLibStringWrapperTryFromStringError {
    #[error("config string wrapper length {len} exceeds maximum {max}")]
    TooLong { len: usize, max: usize },
}
