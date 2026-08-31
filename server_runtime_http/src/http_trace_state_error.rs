#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum HttpTraceStateError {
    #[error("{}", constants_str::TRACESTATE_PRINTABLE_ASCII_MAX_512)]
    Invalid,
}
