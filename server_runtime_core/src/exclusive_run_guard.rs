#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the exclusive-run owner module constructs this private lifecycle guard"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[must_use]
pub struct ExclusiveRunGuard<'run_lt> {
    pub(super) active: &'run_lt super::exclusive_run_atomic_bool::ExclusiveRunAtomicBool,
}
impl Drop for ExclusiveRunGuard<'_> {
    fn drop(&mut self) {
        self.active
            .store(false, std::sync::atomic::Ordering::Release);
    }
}
