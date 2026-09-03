#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_new::New,
)]
#[constructor(pub(crate))]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "cleanup report keeps declaration order aligned with generated layout or processing flow"
)]
pub struct CleanupReport {
    #[getters(copy)]
    batches: crate::cleanup_batch_count::CleanupBatchCount,
    #[getters(copy)]
    rows: crate::cleanup_rows::CleanupRows,
    #[getters(copy)]
    completion: crate::cleanup_completion::CleanupCompletion,
}
