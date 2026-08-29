#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default, newtype::FromInner)]
pub(super) struct StdCollectionsChildProcessMap(
    pub(super)  bounded_types::bounded_b_tree_map::BoundedBTreeMap<
        crate::child_process_id::ChildProcessId,
        crate::child_process_supervisor::ChildProcessSupervisor,
        { usize::MAX },
    >,
);
