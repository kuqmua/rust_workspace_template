use super::LeaseId;

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
pub struct LeaseIds(bounded_types::BoundedVec<LeaseId, 0, { usize::MAX }>);
