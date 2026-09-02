#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum RequestIdTryFromStringError {
    #[error(
        "{}",
        constants_str::REQUEST_ID_MUST_BE_NON_EMPTY_ASCII_UP_TO_128_BYTES
    )]
    Invalid,
}
