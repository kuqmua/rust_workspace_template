use super::{FileStorageRootPathBuf, StoragePathRef};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct SafeFileStorage {
    root: FileStorageRootPathBuf,
}
#[allow(
    clippy::multiple_inherent_impl,
    reason = "domain constructor and path access stay separate from filesystem adapter operations"
)]
impl SafeFileStorage {
    #[must_use]
    pub const fn new(root: FileStorageRootPathBuf) -> Self {
        Self { root }
    }

    pub(crate) fn root(&self) -> StoragePathRef<'_> {
        StoragePathRef::from(self.root.as_ref())
    }
}
