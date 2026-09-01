#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    generate_constructor::New,
)]
#[constructor(pub(crate))]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct CleanupReport {
    #[getters(copy)]
    batches: crate::cleanup_batch_count::CleanupBatchCount,
    #[getters(copy)]
    rows: crate::cleanup_rows::CleanupRows,
    #[getters(copy)]
    completion: crate::cleanup_completion::CleanupCompletion,
}
