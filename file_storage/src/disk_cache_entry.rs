#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::domain_types::{
    DiskCacheModifiedAtSystemTime, StdDiskCacheSize, StorageRelativePathBuf,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DiskCacheEntry {
    pub(super) modified_at: DiskCacheModifiedAtSystemTime,
    pub(super) path: StorageRelativePathBuf,
    pub(super) size: StdDiskCacheSize,
}
impl DiskCacheEntry {
    #[must_use]
    pub const fn new(
        path: StorageRelativePathBuf,
        size: StdDiskCacheSize,
        modified_at: DiskCacheModifiedAtSystemTime,
    ) -> Self {
        Self {
            modified_at,
            path,
            size,
        }
    }
}
