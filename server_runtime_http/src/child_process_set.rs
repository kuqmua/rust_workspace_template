#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct ChildProcessSet {
    maximum: super::ChildProcessSetMaximumNonZeroUsize,
    pub(super) next_id: super::ChildProcessId,
    processes: super::StdCollectionsChildProcessMap,
}

impl ChildProcessSet {
    pub fn insert(
        &mut self,
        process: super::ChildProcessSupervisor,
    ) -> Result<super::ChildProcessId, super::ChildProcessSetError> {
        if self.processes.0.len().get() >= self.maximum.0.get() {
            return Err(super::ChildProcessSetError::Full);
        }
        let id = self.next_id;
        self.next_id = super::ChildProcessId::from(
            self.next_id
                .0
                .checked_add(constants_u64::ONE)
                .ok_or(super::ChildProcessSetError::IdOverflow)?,
        );
        self.processes
            .0
            .try_insert(id, process)
            .map(|_previous| id)
            .map_err(super::ChildProcessSetError::from)
    }

    #[must_use]
    pub fn new(maximum: super::ChildProcessSetMaximumNonZeroUsize) -> Self {
        Self {
            maximum,
            next_id: super::ChildProcessId::from(constants_u64::ZERO),
            processes: super::StdCollectionsChildProcessMap::from(
                bounded_types::BoundedBTreeMap::default(),
            ),
        }
    }

    pub async fn shutdown_all(
        mut self,
        timeout: crate::domain_types::RequestTimeoutDuration,
    ) -> Result<super::ChildProcessReports, super::ChildProcessSetError> {
        let mut reports = Vec::with_capacity(self.processes.0.len().get());
        while let Some((_id, process)) = self.processes.0.pop_first() {
            reports.push(
                process
                    .shutdown(timeout)
                    .await
                    .map_err(super::ChildProcessSetError::Process)?,
            );
        }
        Ok(super::ChildProcessReports::from(
            bounded_types::BoundedVec::from_max_iter(reports),
        ))
    }
}
