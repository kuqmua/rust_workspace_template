#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype::FromInner,
    proc_macro_getters::Getters,
)]
pub struct AdminSharedSemaphoreArc(std::sync::Arc<tokio::sync::Semaphore>);

impl AdminSharedSemaphoreArc {
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Tokio semaphore and Arc constructors are not const
    #[allow(clippy::single_call_fn)] // Arc construction stays inside its cross-thread state owner
    pub(crate) fn new(
        max_concurrent_hashes: crate::runtime_admin_password_hash_concurrency::RuntimeAdminPasswordHashConcurrency,
    ) -> Self {
        Self::from(std::sync::Arc::new(tokio::sync::Semaphore::new(
            max_concurrent_hashes.get().get(),
        )))
    }
}
