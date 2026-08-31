#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, generate_constructor::New)]
#[constructor(pub(crate))]
#[must_use]
pub struct ExclusiveRunGuard<'run_lt> {
    active: &'run_lt super::exclusive_run_atomic_bool::ExclusiveRunAtomicBool,
}
impl Drop for ExclusiveRunGuard<'_> {
    fn drop(&mut self) {
        self.active
            .store(false, std::sync::atomic::Ordering::Release);
    }
}
