#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub struct BoundedReadConcurrencyArcSemaphore(pub(super) std::sync::Arc<tokio::sync::Semaphore>);

impl BoundedReadConcurrencyArcSemaphore {
    #[must_use]
    pub fn new(maximum_concurrent_reads: super::BoundedReadConcurrencyMaximumNonZeroUsize) -> Self {
        Self::from(std::sync::Arc::new(tokio::sync::Semaphore::new(
            maximum_concurrent_reads.0.get(),
        )))
    }
}
