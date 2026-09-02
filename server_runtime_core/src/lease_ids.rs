#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::FromInner,
)]
pub struct LeaseIds(
    bounded_types::bounded_vec::BoundedVec<crate::lease_id::LeaseId, 0, { usize::MAX }>,
);
