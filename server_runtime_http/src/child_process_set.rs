#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct ChildProcessSet {
    maximum: crate::child_process_set_maximum_non_zero_usize::ChildProcessSetMaximumNonZeroUsize,
    next_id: crate::child_process_id::ChildProcessId,
    processes: crate::std_collections_child_process_map::StdCollectionsChildProcessMap,
}

impl ChildProcessSet {
    pub fn insert(
        &mut self,
        child_process_supervisor: crate::child_process_supervisor::ChildProcessSupervisor,
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
            .try_insert(id, child_process_supervisor)
            .map(|_previous| id)
            .map_err(crate::child_process_set_error::ChildProcessSetError::from)
    }

    #[must_use]
    pub fn new(
        child_process_set_maximum_non_zero_usize: crate::child_process_set_maximum_non_zero_usize::ChildProcessSetMaximumNonZeroUsize,
    ) -> Self {
        Self {
            maximum: child_process_set_maximum_non_zero_usize,
            next_id: crate::child_process_id::ChildProcessId::from(constants_u64::ZERO),
            processes:
                crate::std_collections_child_process_map::StdCollectionsChildProcessMap::from(
                    bounded_types::bounded_b_tree_map::BoundedBTreeMap::default(),
                ),
        }
    }

    pub async fn shutdown_all(
        mut self,
        request_timeout_duration: crate::request_timeout_duration::RequestTimeoutDuration,
    ) -> Result<
        crate::child_process_reports::ChildProcessReports,
        crate::child_process_set_error::ChildProcessSetError,
    > {
        let mut reports = Vec::with_capacity(self.processes.len().get());
        while let Some((_id, process)) = self.processes.pop_first() {
            reports.push(
                process
                    .shutdown(request_timeout_duration)
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
        child_process_id: crate::child_process_id::ChildProcessId,
    ) {
        self.next_id = child_process_id;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_process_set_capacity_precedes_overflow_and_failed_insert_preserves_state() {
        let mut child_process_set = crate::child_process_set::ChildProcessSet::new(
            crate::child_process_set_maximum_non_zero_usize::ChildProcessSetMaximumNonZeroUsize::from(std::num::NonZeroUsize::MIN),
        );
        let last_usable_id = crate::child_process_id::ChildProcessId::from(u64::MAX - 1u64);
        child_process_set.set_next_id_for_test(last_usable_id);
        assert!(matches!(
            child_process_set.insert(crate::child_process_supervisor::ChildProcessSupervisor::default()),
            Ok(child_process_id) if child_process_id == last_usable_id
        ));
        assert_eq!(*child_process_set.next_id, u64::MAX);
        assert!(matches!(
            child_process_set
                .insert(crate::child_process_supervisor::ChildProcessSupervisor::default()),
            Err(crate::child_process_set_error::ChildProcessSetError::Full)
        ));
        assert_eq!(child_process_set.processes.len().get(), 1usize);
        assert_eq!(*child_process_set.next_id, u64::MAX);
        assert!(
            matches!(child_process_set.processes.pop_first(), Some((child_process_id, _)) if child_process_id == last_usable_id)
        );
        assert!(matches!(
            child_process_set
                .insert(crate::child_process_supervisor::ChildProcessSupervisor::default()),
            Err(crate::child_process_set_error::ChildProcessSetError::IdOverflow)
        ));
        assert_eq!(child_process_set.processes.len().get(), 0usize);
        assert_eq!(*child_process_set.next_id, u64::MAX);
    }
}
