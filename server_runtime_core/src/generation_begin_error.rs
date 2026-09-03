#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum GenerationBeginError {
    #[error("{}", constants_str::GENERATION_OVERFLOW)]
    Overflow(crate::generation::Generation),
}
