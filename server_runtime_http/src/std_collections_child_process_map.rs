#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Default,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_deref_mut_inner::DerefMutInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct StdCollectionsChildProcessMap(
    bounded_types::bounded_b_tree_map::BoundedBTreeMap<
        crate::child_process_id::ChildProcessId,
        crate::child_process_supervisor::ChildProcessSupervisor,
        { usize::MAX },
    >,
);
