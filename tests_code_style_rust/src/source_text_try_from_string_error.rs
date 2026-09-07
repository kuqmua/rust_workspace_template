#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, thiserror::Error,
)]
pub(super) enum SourceTextTryFromStringError {
    #[error(
        "source text length {} exceeds max {}",
        .len.get(),
        constants_usize::VALUE_16_777_216
    )]
    TooLong {
        len: crate::analyzer_count::AnalyzerCount,
    },
}
