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
    #[allow(
        clippy::missing_const_for_fn,
        reason = "lint suppression is required here"
    )]
    #[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
    pub(crate) fn new(
        runtime_admin_password_hash_concurrency: crate::runtime_admin_password_hash_concurrency::RuntimeAdminPasswordHashConcurrency,
    ) -> Self {
        Self::from(std::sync::Arc::new(tokio::sync::Semaphore::new(
            runtime_admin_password_hash_concurrency.get().get(),
        )))
    }
}
