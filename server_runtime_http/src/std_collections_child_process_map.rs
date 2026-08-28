#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default, newtype::FromInner)]
pub(super) struct StdCollectionsChildProcessMap(
    pub(super)  bounded_types::BoundedBTreeMap<
        super::ChildProcessId,
        super::ChildProcessSupervisor,
        { usize::MAX },
    >,
);
