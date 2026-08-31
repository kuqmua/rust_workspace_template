#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Default,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
)]
pub(super) struct StdCollectionsChildProcessMap(
    bounded_types::bounded_b_tree_map::BoundedBTreeMap<
        crate::child_process_id::ChildProcessId,
        crate::child_process_supervisor::ChildProcessSupervisor,
        { usize::MAX },
    >,
);
