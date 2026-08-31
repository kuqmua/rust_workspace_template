#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner, newtype::IntoInner,
)]
pub struct ArcTokioSemaphore(std::sync::Arc<tokio::sync::Semaphore>);

impl ArcTokioSemaphore {
    #[must_use]
    pub fn new(
        permit_count: crate::semaphore_permit_count_non_zero_usize::SemaphorePermitCountNonZeroUsize,
    ) -> Self {
        Self::from(std::sync::Arc::new(tokio::sync::Semaphore::new(
            permit_count.get(),
        )))
    }

    #[must_use]
    pub fn try_acquire(
        &self,
    ) -> Option<crate::tokio_owned_semaphore_permit::TokioOwnedSemaphorePermit> {
        std::sync::Arc::clone(&self.0)
            .try_acquire_owned()
            .ok()
            .map(crate::tokio_owned_semaphore_permit::TokioOwnedSemaphorePermit::from)
    }
}
