#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default, newtype::FromInner)]
pub(super) struct StdCollectionsChildProcessMap(
    pub(super)  bounded_types::domain_types::btree::BoundedBTreeMap<
        super::ChildProcessId,
        super::ChildProcessSupervisor,
        { usize::MAX },
    >,
);
