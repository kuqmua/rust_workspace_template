#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum InitStringError {
    #[error("environment initializer string value is invalid")]
    Invalid,
}
