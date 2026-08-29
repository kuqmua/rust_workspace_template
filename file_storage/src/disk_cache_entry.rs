#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DiskCacheEntry {
    pub(super) modified_at:
        crate::disk_cache_modified_at_system_time::DiskCacheModifiedAtSystemTime,
    pub(super) path: crate::storage_relative_path_buf::StorageRelativePathBuf,
    pub(super) size: crate::std_disk_cache_size::StdDiskCacheSize,
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
}
