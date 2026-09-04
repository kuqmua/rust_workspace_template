#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[getters(bare)]
pub struct DiskCacheEntry {
    #[constructor(order = 2)]
    #[getters(copy)]
    modified_at: crate::disk_cache_modified_at_system_time::DiskCacheModifiedAtSystemTime,
    #[constructor(order = 0)]
    path: crate::storage_relative_path_buf::StorageRelativePathBuf,
    #[constructor(order = 1)]
    #[getters(copy)]
    size: crate::std_disk_cache_size::StdDiskCacheSize,
}
