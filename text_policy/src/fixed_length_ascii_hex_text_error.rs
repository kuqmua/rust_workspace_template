#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum FixedLengthAsciiHexTextError {
    #[error("hexadecimal text has an unexpected length")]
    InvalidLength,
    #[error("hexadecimal text must contain only lowercase ASCII hexadecimal digits")]
    InvalidSymbol,
}
