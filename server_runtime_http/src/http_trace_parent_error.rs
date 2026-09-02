#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum HttpTraceParentError {
    #[error("{}", constants_str::TRACEPARENT_W3C_VERSION_00_FORMAT)]
    Format,
    #[error("{}", constants_str::TRACEPARENT_PARENT_ID_NOT_ZERO)]
    ZeroParentId,
    #[error("{}", constants_str::TRACEPARENT_TRACE_ID_NOT_ZERO)]
    ZeroTraceId,
}
