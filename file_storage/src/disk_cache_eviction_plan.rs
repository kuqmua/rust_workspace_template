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
    bounded_types::bounded_vec::BoundedVec<
        crate::storage_relative_path_buf::StorageRelativePathBuf,
        0,
        { usize::MAX },
    >,
);
