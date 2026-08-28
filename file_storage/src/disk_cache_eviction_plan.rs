use crate::domain_types::StorageRelativePathBuf;

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
pub struct DiskCacheEvictionPlan(
    bounded_types::BoundedVec<StorageRelativePathBuf, 0, { usize::MAX }>,
);
