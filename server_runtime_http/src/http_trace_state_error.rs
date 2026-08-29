#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{}", constants_str::test_fixtures::TRACESTATE_PRINTABLE_ASCII_MAX_512)]
pub struct HttpTraceStateError;
