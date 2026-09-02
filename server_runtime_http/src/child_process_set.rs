#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct ChildProcessSet {
    maximum: crate::child_process_set_maximum_non_zero_usize::ChildProcessSetMaximumNonZeroUsize,
    next_id: crate::child_process_id::ChildProcessId,
    processes: crate::std_collections_child_process_map::StdCollectionsChildProcessMap,
}

impl ChildProcessSet {
    pub fn insert(
        &mut self,
        process: crate::child_process_supervisor::ChildProcessSupervisor,
    ) -> Result<
        crate::child_process_id::ChildProcessId,
        crate::child_process_set_error::ChildProcessSetError,
    > {
        if self.processes.len().get() >= self.maximum.get() {
            return Err(crate::child_process_set_error::ChildProcessSetError::Full);
        }
        let id = self.next_id;
        self.next_id = crate::child_process_id::ChildProcessId::from(
            (*self.next_id)
                .checked_add(constants_u64::ONE)
                .ok_or(crate::child_process_set_error::ChildProcessSetError::IdOverflow)?,
        );
        self.processes
            .try_insert(id, process)
            .map(|_previous| id)
            .map_err(crate::child_process_set_error::ChildProcessSetError::from)
    }

    #[must_use]
    pub fn new(
        maximum: crate::child_process_set_maximum_non_zero_usize::ChildProcessSetMaximumNonZeroUsize,
    ) -> Self {
        Self {
            maximum,
            next_id: crate::child_process_id::ChildProcessId::from(constants_u64::ZERO),
            processes:
                crate::std_collections_child_process_map::StdCollectionsChildProcessMap::from(
                    bounded_types::bounded_b_tree_map::BoundedBTreeMap::default(),
                ),
        }
    }

    pub async fn shutdown_all(
        mut self,
        timeout: crate::request_timeout_duration::RequestTimeoutDuration,
    ) -> Result<
        crate::child_process_reports::ChildProcessReports,
        crate::child_process_set_error::ChildProcessSetError,
    > {
        let mut reports = Vec::with_capacity(self.processes.len().get());
        while let Some((_id, process)) = self.processes.pop_first() {
            reports.push(
                process
                    .shutdown(timeout)
                    .await
                    .map_err(crate::child_process_set_error::ChildProcessSetError::Process)?,
            );
        }
        Ok(crate::child_process_reports::ChildProcessReports::from(
            bounded_types::bounded_vec::BoundedVec::from_max_iter(reports),
        ))
    }

    #[cfg(test)]
    pub(crate) const fn set_next_id_for_test(
        &mut self,
        value: crate::child_process_id::ChildProcessId,
    ) {
        self.next_id = value;
    }
}
