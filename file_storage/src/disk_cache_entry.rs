#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_new::New,
)]
pub struct DiskCacheEntry {
    #[constructor(order = 2)]
    modified_at: crate::disk_cache_modified_at_system_time::DiskCacheModifiedAtSystemTime,
    #[constructor(order = 0)]
    path: crate::storage_relative_path_buf::StorageRelativePathBuf,
    #[constructor(order = 1)]
    size: crate::std_disk_cache_size::StdDiskCacheSize,
}
impl DiskCacheEntry {
    #[must_use]
    pub const fn parts(
        &self,
    ) -> (
        crate::disk_cache_modified_at_system_time::DiskCacheModifiedAtSystemTime,
        &crate::storage_relative_path_buf::StorageRelativePathBuf,
        crate::std_disk_cache_size::StdDiskCacheSize,
    ) {
        (self.modified_at, &self.path, self.size)
    }
}
