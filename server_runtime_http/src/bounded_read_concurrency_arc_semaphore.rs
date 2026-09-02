#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInner,
)]
pub struct BoundedReadConcurrencyArcSemaphore(std::sync::Arc<tokio::sync::Semaphore>);

impl BoundedReadConcurrencyArcSemaphore {
    #[must_use]
    pub fn new(
        bounded_read_concurrency_maximum_non_zero_usize: crate::bounded_read_concurrency_maximum_non_zero_usize::BoundedReadConcurrencyMaximumNonZeroUsize,
    ) -> Self {
        Self::from(std::sync::Arc::new(tokio::sync::Semaphore::new(
            bounded_read_concurrency_maximum_non_zero_usize.get(),
        )))
    }
}
