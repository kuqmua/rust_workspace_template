#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct LeaseIds(
    bounded_types::bounded_vec::BoundedVec<crate::lease_id::LeaseId, 0, { usize::MAX }>,
);
