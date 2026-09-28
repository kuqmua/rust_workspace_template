#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum TryFromStdEnvVarOkTracingFormatError {
    #[error("{}", constants_str::CONFIG_TRACING_FORMAT_MUST_BE_JSON_OR_TEXT)]
    Unknown,
}
