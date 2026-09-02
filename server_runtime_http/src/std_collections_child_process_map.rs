#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Default,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::DerefMutInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct StdCollectionsChildProcessMap(
    bounded_types::bounded_b_tree_map::BoundedBTreeMap<
        crate::child_process_id::ChildProcessId,
        crate::child_process_supervisor::ChildProcessSupervisor,
        { usize::MAX },
    >,
);
