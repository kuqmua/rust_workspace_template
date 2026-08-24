#[derive(optml::Optml, Debug)]
pub struct ExclusiveRun {
    active: StdExclusiveRunAtomicBool,
}
impl ExclusiveRun {
    #[must_use]
    pub fn new() -> Self {
        Self {
            active: StdExclusiveRunAtomicBool::from(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    pub fn try_acquire(&self) -> Result<ExclusiveRunGuard<'_>, ExclusiveRunAlreadyActive> {
        self.active
            .0
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

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("operation is already running")]
pub struct ExclusiveRunAlreadyActive;

#[derive(optml::Optml, Debug)]
#[must_use]
pub struct ExclusiveRunGuard<'run_lt> {
    active: &'run_lt StdExclusiveRunAtomicBool,
}
impl Drop for ExclusiveRunGuard<'_> {
    fn drop(&mut self) {
        self.active
            .0
            .store(false, std::sync::atomic::Ordering::Release);
    }
}

#[derive(optml::Optml, Debug, newtype::FromInner)]
struct StdExclusiveRunAtomicBool(std::sync::atomic::AtomicBool);

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
