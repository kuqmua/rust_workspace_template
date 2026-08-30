#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DiskCacheEntry {
    modified_at: crate::disk_cache_modified_at_system_time::DiskCacheModifiedAtSystemTime,
    path: crate::storage_relative_path_buf::StorageRelativePathBuf,
    size: crate::std_disk_cache_size::StdDiskCacheSize,
}
impl DiskCacheEntry {
    #[must_use]
    pub const fn new(
        path: crate::storage_relative_path_buf::StorageRelativePathBuf,
        size: crate::std_disk_cache_size::StdDiskCacheSize,
        modified_at: crate::disk_cache_modified_at_system_time::DiskCacheModifiedAtSystemTime,
    ) -> Self {
        Self {
            modified_at,
            path,
            size,
        }
    }

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
