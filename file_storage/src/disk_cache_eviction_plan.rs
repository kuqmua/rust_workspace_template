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
pub struct DiskCacheEvictionPlan(
    bounded_types::bounded_vec::BoundedVec<
        crate::storage_relative_path_buf::StorageRelativePathBuf,
        0,
        { usize::MAX },
    >,
);
