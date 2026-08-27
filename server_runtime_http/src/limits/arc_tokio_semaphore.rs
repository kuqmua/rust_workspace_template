#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub struct ArcTokioSemaphore(pub(super) std::sync::Arc<tokio::sync::Semaphore>);

impl ArcTokioSemaphore {
    #[must_use]
    pub fn new(permit_count: super::SemaphorePermitCountNonZeroUsize) -> Self {
        Self::from(std::sync::Arc::new(tokio::sync::Semaphore::new(
            permit_count.0.get(),
        )))
    }

    #[must_use]
    pub fn try_acquire(&self) -> Option<super::TokioOwnedSemaphorePermit> {
        std::sync::Arc::clone(&self.0)
            .try_acquire_owned()
            .ok()
            .map(super::TokioOwnedSemaphorePermit::from)
    }
}
