#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum PasswordLengthRangeError {
    #[error("password maximum length must not be less than minimum length")]
    Invalid,
}
