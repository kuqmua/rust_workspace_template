#[path = "exclusive_run_already_active.rs"]
mod exclusive_run_already_active;
#[path = "exclusive_run_atomic_bool.rs"]
mod exclusive_run_atomic_bool;
#[path = "exclusive_run_guard.rs"]
mod exclusive_run_guard;

pub use exclusive_run_already_active::ExclusiveRunAlreadyActive;
pub use exclusive_run_guard::ExclusiveRunGuard;
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct ExclusiveRun {
    active: exclusive_run_atomic_bool::ExclusiveRunAtomicBool,
}
impl ExclusiveRun {
    #[must_use]
    pub fn new() -> Self {
        Self {
            active: exclusive_run_atomic_bool::ExclusiveRunAtomicBool::from(
                std::sync::atomic::AtomicBool::new(false),
            ),
        }
    }

    pub fn try_acquire(&self) -> Result<ExclusiveRunGuard<'_>, ExclusiveRunAlreadyActive> {
        self.active
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::AcqRel,
                std::sync::atomic::Ordering::Acquire,
            )
            .map(|_previous| ExclusiveRunGuard {
                active: &self.active,
            })
            .map_err(|_active| ExclusiveRunAlreadyActive)
    }
}
impl Default for ExclusiveRun {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn guard_prevents_overlap_and_releases_on_drop() {
        let run = super::ExclusiveRun::new();
        let guard = run
            .try_acquire()
            .expect("9b776c85 guard_prevents_overlap_and_releases_on_drop invariant must hold");
        assert!(matches!(
            run.try_acquire(),
            Err(super::ExclusiveRunAlreadyActive)
        ));
        drop(guard);
        let _next_guard = run
            .try_acquire()
            .expect("d43a617d guard_prevents_overlap_and_releases_on_drop invariant must hold");
    }
}
