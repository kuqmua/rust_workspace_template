#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error(
    "{}",
    constants_str::catalog::REQUEST_ID_MUST_BE_NON_EMPTY_ASCII_UP_TO_128_BYTES
)]
pub struct RequestIdTryFromStringError;
